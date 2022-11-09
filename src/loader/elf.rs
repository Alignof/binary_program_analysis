mod elf_32;
mod elf_64;

use memmap::Mmap;
use crate::loader::Loader;
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
    pub elf_header: ElfHeader64,
    pub prog_headers: Vec<ProgramHeader64>,
    pub sect_headers: Vec<SectionHeader64>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    pub fn new(mapped_data: Mmap) -> Box<dyn Loader> {
        let new_elf = ElfHeader64::new(&mapped_data);
        let new_prog = ProgramHeader64::new(&mapped_data, &new_elf);
        let new_sect = SectionHeader64::new(&mapped_data, &new_elf);

        Box::new(
            ElfLoader {
                elf_header: new_elf,
                prog_headers: new_prog,
                sect_headers: new_sect,
                mem_data: mapped_data,
            }
        )
    }
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
}
