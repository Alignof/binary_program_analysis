use crate::loader::elf::{ElfHeader, ElfIdentification};
use crate::loader::{get_u16, get_u32};

fn get_elf_type_name(elf_type: u16) -> &'static str {
    match elf_type {
        0 => "ET_NONE",
        1 => "ET_REL",
        2 => "ET_EXEC",
        3 => "ET_DYN",
        4 => "ET_CORE",
        _ => "unknown type",
    }
}

pub struct ElfHeader32 {
    e_ident: ElfIdentification,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ElfHeader32 {
    pub fn new(mmap: &[u8], elf_ident: ElfIdentification) -> Box<Self> {
        const ELF_HEADER_START: usize = 16;
        Box::new(ElfHeader32 {
            e_ident: elf_ident,
            e_type: get_u16(mmap, ELF_HEADER_START),
            e_machine: get_u16(mmap, ELF_HEADER_START + 2),
            e_version: get_u32(mmap, ELF_HEADER_START + 4),
            e_entry: get_u32(mmap, ELF_HEADER_START + 8),
            e_phoff: get_u32(mmap, ELF_HEADER_START + 12),
            e_shoff: get_u32(mmap, ELF_HEADER_START + 16),
            e_flags: get_u32(mmap, ELF_HEADER_START + 20),
            e_ehsize: get_u16(mmap, ELF_HEADER_START + 24),
            e_phentsize: get_u16(mmap, ELF_HEADER_START + 26),
            e_phnum: get_u16(mmap, ELF_HEADER_START + 28),
            e_shentsize: get_u16(mmap, ELF_HEADER_START + 30),
            e_shnum: get_u16(mmap, ELF_HEADER_START + 32),
            e_shstrndx: get_u16(mmap, ELF_HEADER_START + 34),
        })
    }
}

impl ElfHeader for ElfHeader32 {
    fn show(&self) {
        println!("================ elf header ================");
        self.e_ident.show();
        println!("e_type:\t\t{}", get_elf_type_name(self.e_type));
        println!("e_machine:\t{}", self.e_machine);
        println!("e_version:\t0x{:x}", self.e_version);
        println!("e_entry:\t0x{:x?}", self.e_entry);
        println!("e_phoff:\t{} (bytes into file)", self.e_phoff);
        println!("e_shoff:\t{} (bytes into file)", self.e_shoff);
        println!("e_flags:\t0x{:x}", self.e_flags);
        println!("e_ehsize:\t{} (bytes)", self.e_ehsize);
        println!("e_phentsize:\t{} (bytes)", self.e_phentsize);
        println!("e_phnum:\t{}", self.e_phnum);
        println!("e_shentsize:\t{} (bytes)", self.e_shentsize);
        println!("e_shnum:\t{}", self.e_shnum);
        println!("e_shstrndx:\t{}", self.e_shstrndx);
    }
}
