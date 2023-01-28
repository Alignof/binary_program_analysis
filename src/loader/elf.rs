mod elf_32;
mod elf_64;

use crate::loader::{get_u32, get_u64, Arch, Function, Loader};
use elf_32::elf_header::ElfHeader32;
use elf_32::program_header::ProgramHeader32;
use elf_32::section_header::SectionHeader32;
use elf_64::elf_header::ElfHeader64;
use elf_64::program_header::ProgramHeader64;
use elf_64::section_header::SectionHeader64;
use memmap::Mmap;
use std::collections::HashMap;

pub struct ElfIdentification {
    magic: [u8; 16],
    class: u8,
    endian: u8,
    version: u8,
    os_abi: u8,
    os_abi_ver: u8,
}

impl ElfIdentification {
    fn new(mmap: &[u8]) -> ElfIdentification {
        let mut magic: [u8; 16] = [0; 16];
        for (i, m) in mmap[0..16].iter().enumerate() {
            magic[i] = *m;
        }

        ElfIdentification {
            magic,
            class: mmap[4],
            endian: mmap[5],
            version: mmap[6],
            os_abi: mmap[7],
            os_abi_ver: mmap[8],
        }
    }

    fn get_arch(&self) -> Arch {
        const EI_CLASS: usize = 4;
        if self.magic[EI_CLASS] == 1 {
            Arch::Bit32
        } else {
            Arch::Bit64
        }
    }

    fn show(&self) {
        print!("magic:\t");
        for byte in self.magic.iter() {
            print!("{:02x} ", byte);
        }
        println!();
        println!("class:\t\t{:?}", self.class);
        println!("endian:\t\t{:?}", self.endian);
        println!("version:\t{:?}", self.version);
        println!("os_abi:\t\t{:?}", self.os_abi);
        println!("os_abi_ver:\t{:?}", self.os_abi_ver);
    }
}

pub struct ElfLoader {
    pub elf_header: Box<dyn ElfHeader>,
    pub prog_headers: Vec<Box<dyn ProgramHeader>>,
    pub sect_headers: Vec<Box<dyn SectionHeader>>,
    pub functions: Vec<Function>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    fn addr2offset(prog_headers: &[Box<dyn ProgramHeader>], addr: u64) -> Option<u64> {
        let mut addr_table = Vec::new();
        for seg in prog_headers {
            addr_table.push(seg.offset_and_addr());
        }

        eprintln!("{addr:x}");
        addr_table.sort_by(|x, y| (x.1).cmp(&y.1));
        for w in addr_table.windows(2) {
            let (a, z) = (w[0], w[1]);
            if a.1 <= addr && addr < z.1 {
                return Some(a.0 + (addr - a.1));
            }
        }

        None
    }

    fn create_func_table(
        mmap: &[u8],
        arch: Arch,
        prog_headers: &[Box<dyn ProgramHeader>],
        sect_headers: &[Box<dyn SectionHeader>],
    ) -> Vec<Function> {
        let symtab = sect_headers.iter().find(|s| s.sh_name() == ".symtab");
        let strtab = sect_headers.iter().find(|s| s.sh_name() == ".strtab");

        let st_size: usize = match arch {
            Arch::Bit32 => 16,
            Arch::Bit64 => 24,
        };
        let mut functions: Vec<Function> = Vec::new();
        if let (Some(symtab), Some(strtab)) = (symtab, strtab) {
            for symtab_off in symtab.section_range().step_by(st_size) {
                let st_info = mmap[symtab_off as usize + 4];
                if st_info & 0xf == 2 {
                    let st_name_off = get_u32(mmap, symtab_off as usize);
                    let st_name = mmap[(strtab.sh_offset() + st_name_off as u64) as usize..]
                        .iter()
                        .take_while(|c| **c as char != '\0')
                        .map(|c| *c as char)
                        .collect::<String>();
                    let st_addr = get_u64(mmap, (symtab_off + 8) as usize);
                    let st_size = get_u64(mmap, (symtab_off + 16) as usize);

                    functions.push(Function {
                        name: st_name,
                        addr: Self::addr2offset(prog_headers, st_addr).unwrap(),
                        size: st_size,
                    });
                }
            }
        }

        functions
    }

    pub fn new(mapped_data: Mmap) -> Box<dyn Loader> {
        let elf_ident = ElfIdentification::new(&mapped_data);
        match elf_ident.get_arch() {
            Arch::Bit32 => {
                let new_elf = ElfHeader32::new(&mapped_data, elf_ident);
                let new_prog = ProgramHeader32::new(&mapped_data, &new_elf);
                let new_sect = SectionHeader32::new(&mapped_data, &new_elf);
                let new_func =
                    Self::create_func_table(&mapped_data, Arch::Bit32, &new_prog, &new_sect);

                Box::new(ElfLoader {
                    elf_header: new_elf,
                    prog_headers: new_prog,
                    sect_headers: new_sect,
                    functions: new_func,
                    mem_data: mapped_data,
                })
            }
            Arch::Bit64 => {
                let new_elf = ElfHeader64::new(&mapped_data, elf_ident);
                let new_prog = ProgramHeader64::new(&mapped_data, &new_elf);
                let new_sect = SectionHeader64::new(&mapped_data, &new_elf);
                let new_func =
                    Self::create_func_table(&mapped_data, Arch::Bit64, &new_prog, &new_sect);

                Box::new(ElfLoader {
                    elf_header: new_elf,
                    prog_headers: new_prog,
                    sect_headers: new_sect,
                    functions: new_func,
                    mem_data: mapped_data,
                })
            }
        }
    }
}

pub trait ElfHeader {
    fn show(&self);
}

pub trait ProgramHeader {
    fn show(&self, id: usize);
    fn dump(&self, mmap: &[u8]);
    fn offset_and_addr(&self) -> (u64, u64);
}

pub trait SectionHeader {
    fn get_sh_name(mmap: &[u8], section_head: usize, name_table_head: usize) -> String
    where
        Self: Sized;
    fn sh_name(&self) -> &str;
    fn sh_offset(&self) -> u64;
    fn section_range(&self) -> std::ops::Range<u64>;
    fn type_to_str(&self) -> &'static str;
    fn show(&self, id: usize);
    fn dump(&self, mmap: &[u8]);
    fn inst_analysis(&self, inst_list: &mut HashMap<String, u32>, mmap: &[u8]);
}

impl Loader for ElfLoader {
    fn header_show(&self) {
        self.elf_header.show();
    }

    fn show_segment(&self) {
        for (id, prog) in self.prog_headers.iter().enumerate() {
            prog.show(id);
            prog.dump(&self.mem_data);
            println!("\n\n");
        }
    }

    fn show_section(&self) {
        for (id, sect) in self.sect_headers.iter().enumerate() {
            sect.show(id);
            println!("\n\n");
        }
    }

    fn disassemble(&self) {
        for (id, sect) in self.sect_headers.iter().enumerate() {
            sect.show(id);
            sect.dump(&self.mem_data);
            println!("\n\n");
        }
    }

    fn show_all_header(&self) {
        self.elf_header.show();

        println!("\n\n");

        for (id, prog) in self.prog_headers.iter().enumerate() {
            prog.show(id);
        }

        println!("\n\n");

        for (id, sect) in self.sect_headers.iter().enumerate() {
            sect.show(id);
        }
    }

    fn analysis(&self) {
        let mut inst_list_overall = HashMap::new();
        for func in self.functions.iter() {
            let mut inst_list = HashMap::new();
            let call_addrs = func.inst_analysis(&mut inst_list, &self.mem_data);

            for (name, count) in inst_list.clone() {
                *inst_list_overall.entry(name).or_insert(0) += count;
            }

            let mut inst_list = inst_list.iter().collect::<Vec<(&String, &i32)>>();
            inst_list.sort_by(|a, b| (-(a.1)).cmp(&(-(b.1))));
            for t in inst_list.iter() {
                println!("{}: {}", t.0, t.1);
            }

            if !call_addrs.is_empty() {
                print!("calling functions: ");
                for call_addr in call_addrs {
                    let call_func = self
                        .functions
                        .iter()
                        .find_map(|f| {
                            if f.addr == call_addr {
                                Some(f.name.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or(format!("{}", call_addr));
                    print!("func_{} ", call_func);
                }
                println!();
            }
            println!();
        }

        println!("======================");
        let mut inst_count = 0;
        let mut inst_list_overall = inst_list_overall.iter().collect::<Vec<(&String, &i32)>>();
        inst_list_overall.sort_by(|a, b| (-(a.1)).cmp(&(-(b.1))));
        for t in inst_list_overall.iter() {
            inst_count += t.1;
            println!("{}: {}", t.0, t.1);
        }
        println!("-----");
        println!("instructions: {}", inst_count);
        println!("======================");
    }

    fn byte_histogram(&self) {
        use plotters::prelude::*;

        let mut histogram = (0..255)
            .collect::<Vec<u8>>()
            .iter()
            .map(|x| (*x, 0_u32))
            .collect::<HashMap<u8, u32>>();

        for m in self.mem_data.iter() {
            *histogram.entry(*m).or_insert(0) += 1;
        }
        let max_count: u32 = *histogram.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;

        // calc entropy
        let mut entropy: f32 = 0.0;
        for (_, count) in histogram.iter() {
            let p: f32 = (*count as f32) / (self.mem_data.len() as f32);
            entropy -= p * p.log(2.0);
        }
        println!("entropy: {}", entropy);

        // let mut histogram = histogram.iter().collect::<Vec<(&u8, &u32)>>();
        // histogram.sort_by(|a, b| b.1.cmp(a.1));
        // dbg!(histogram);

        let root = BitMapBackend::new("target/histogram.png", (1080, 720)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .margin(10)
            .caption("Byte histogram", ("sans-serif", 25.0))
            .build_cartesian_2d((0u32..255u32).into_segmented(), 0u32..max_count)
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(BLACK.mix(0.5))
            .y_desc("Count")
            .x_desc("Byte")
            .axis_desc_style(("sans-serif", 15))
            .draw()
            .unwrap();

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(RED.filled())
                    .margin(0)
                    .data(histogram.iter().map(|(x, y)| (*x as u32, *y))),
            )
            .unwrap();
    }
}
