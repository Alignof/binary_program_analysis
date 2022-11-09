mod elf_32;
mod elf_64;

use std::collections::HashMap;
use memmap::Mmap;
use crate::loader::{get_u32, get_u64, Loader, Function};
use elf_64::elf_header::ElfHeader64;
use elf_64::program_header::ProgramHeader64;
use elf_64::section_header::SectionHeader64;

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
    pub prog_headers: Vec<ProgramHeader64>,
    pub sect_headers: Vec<SectionHeader64>,
    pub functions: Vec<Function>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    fn create_func_table(mmap: &[u8], sect_headers: &Vec<SectionHeader64>) -> Vec<Function> {
        let symtab = sect_headers.iter()
            .find_map(|s| {
                if s.sh_name == ".symtab" {
                    return Some(s);
                }
                None
            });
        let strtab = sect_headers.iter()
            .find_map(|s| {
                if s.sh_name == ".strtab" {
                    return Some(s);
                }
                None
            });

        const ST_SIZE: usize = 24;
        let mut functions: Vec<Function> = Vec::new();
        if let (Some(symtab), Some(strtab)) = (symtab, strtab) {
            for symtab_off in (symtab.sh_offset .. symtab.sh_offset + symtab.sh_size).step_by(ST_SIZE) {
                let st_info = mmap[symtab_off as usize + 4];
                if st_info & 0xf == 2 {
                    let st_name_off = get_u32(mmap, symtab_off as usize);
                    let st_name = mmap[(strtab.sh_offset + st_name_off as u64) as usize ..]
                        .iter()
                        .take_while(|c| **c as char != '\0')
                        .map(|c| *c as char)
                        .collect::<String>();
                    let st_addr = get_u64(mmap, (symtab_off + 8) as usize);
                    let st_size = get_u64(mmap, (symtab_off + 16) as usize);

                    functions.push(
                        Function {
                            name: st_name,
                            addr: st_addr,
                            size: st_size,
                        }
                    );
                }
            }
        }

        functions
    }

    pub fn new(mapped_data: Mmap) -> Box<dyn Loader> {
        let new_elf = ElfHeader64::new(&mapped_data);
        let new_prog = ProgramHeader64::new(&mapped_data, &new_elf);
        let new_sect = SectionHeader64::new(&mapped_data, &new_elf);
        let new_func = Self::create_func_table(&mapped_data, &new_sect);

        Box::new(
            ElfLoader {
                elf_header: new_elf,
                prog_headers: new_prog,
                sect_headers: new_sect,
                functions: new_func,
                mem_data: mapped_data,
            }
        )
    }
}

trait ElfHeader {
    fn show(&self);
}

trait ProgramHeader {
    fn show(&self, id: usize);
    fn dump(&self, mmap: &[u8]);
}

trait SectionHeader {
    fn get_sh_name(mmap: &[u8], section_head: usize, name_table_head: usize) -> String;
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
        for func in self.functions.iter() {
            let mut inst_list = HashMap::new();
            func.inst_analysis(&mut inst_list, &self.mem_data);
            println!("{:#?}", inst_list);
        }
    }
}
