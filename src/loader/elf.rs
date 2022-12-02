mod elf_32;
mod elf_64;

use crate::loader::{get_u32, get_u64, Function, Loader};
use elf_64::elf_header::ElfHeader64;
use elf_64::program_header::ProgramHeader64;
use elf_64::section_header::SectionHeader64;
use memmap::Mmap;
use std::collections::HashMap;

struct ElfIdentification {
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

        addr_table.sort_by(|x, y| (x.1).cmp(&y.1));
        for w in (&addr_table).windows(2) {
            let (a, z) = (w[0], w[1]);
            if a.1 <= addr && addr < z.1 {
                return Some(a.0 + (addr - a.1));
            }
        }

        None
    }

    fn create_func_table(
        mmap: &[u8],
        prog_headers: &[Box<dyn ProgramHeader>],
        sect_headers: &[Box<dyn SectionHeader>],
    ) -> Vec<Function> {
        let symtab = sect_headers.iter().find(|s| s.sh_name() == ".symtab");
        let strtab = sect_headers.iter().find(|s| s.sh_name() == ".strtab");

        const ST_SIZE: usize = 24;
        let mut functions: Vec<Function> = Vec::new();
        if let (Some(symtab), Some(strtab)) = (symtab, strtab) {
            for symtab_off in symtab.section_range().step_by(ST_SIZE) {
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
        let new_elf = ElfHeader64::new(&mapped_data);
        let new_prog = ProgramHeader64::new(&mapped_data, &new_elf);
        let new_sect = SectionHeader64::new(&mapped_data, &new_elf);
        let new_func = Self::create_func_table(&mapped_data, &new_prog, &new_sect);

        Box::new(ElfLoader {
            elf_header: new_elf,
            prog_headers: new_prog,
            sect_headers: new_sect,
            functions: new_func,
            mem_data: mapped_data,
        })
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
}
