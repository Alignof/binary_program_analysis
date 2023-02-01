use super::ElfHeader32;
use crate::loader::elf::SectionHeader;
use crate::loader::get_u32;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};
use std::collections::HashMap;

pub struct SectionHeader32 {
    pub sh_name: String,
    sh_type: u32,
    sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}

impl SectionHeader32 {
    pub fn new(mmap: &[u8], elf_header: &ElfHeader32) -> Vec<Box<dyn SectionHeader>> {
        let mut new_sect: Vec<Box<dyn SectionHeader>> = Vec::new();
        let name_table =
            elf_header.e_shoff + (elf_header.e_shentsize * elf_header.e_shstrndx) as u32;
        let name_table_off: usize = get_u32(mmap, (name_table as usize) + 16) as usize;

        for section_num in 0..elf_header.e_shnum {
            let section_head: usize =
                (elf_header.e_shoff + (elf_header.e_shentsize * section_num) as u32) as usize;

            new_sect.push(Box::new(SectionHeader32 {
                sh_name: SectionHeader32::get_sh_name(mmap, section_head, name_table_off),
                sh_type: get_u32(mmap, section_head + 4),
                sh_flags: get_u32(mmap, section_head + 8),
                sh_addr: get_u32(mmap, section_head + 12),
                sh_offset: get_u32(mmap, section_head + 16),
                sh_size: get_u32(mmap, section_head + 20),
                sh_link: get_u32(mmap, section_head + 24),
                sh_info: get_u32(mmap, section_head + 28),
                sh_addralign: get_u32(mmap, section_head + 32),
                sh_entsize: get_u32(mmap, section_head + 34),
            }));
        }

        new_sect
    }
}

impl SectionHeader for SectionHeader32 {
    fn get_sh_name(mmap: &[u8], section_head: usize, name_table_head: usize) -> String {
        let name_id: usize = get_u32(mmap, section_head) as usize;
        let mut sh_name: String = String::new();

        for c in mmap[name_table_head + name_id..].iter() {
            if *c as char == '\0' {
                break;
            }
            sh_name.push(*c as char);
        }

        sh_name
    }

    fn sh_name(&self) -> &str {
        &self.sh_name
    }

    fn sh_offset(&self) -> u64 {
        self.sh_offset as u64
    }

    fn section_range(&self) -> std::ops::Range<u64> {
        self.sh_offset as u64..(self.sh_offset + self.sh_size) as u64
    }

    fn type_to_str(&self) -> &'static str {
        match self.sh_type {
            0 => "SHT_NULL",
            1 => "SHT_PROGBITS",
            2 => "SHT_SYMTAB",
            3 => "SHT_STRTAB",
            4 => "SHT_RELA",
            5 => "SHT_HASH",
            6 => "SHT_DYNAMIC",
            7 => "SHT_NOTE",
            8 => "SHT_NOBITS",
            9 => "SHT_REL",
            10 => "SHT_SHLIB",
            11 => "SHT_DYNSYM",
            12 => "SHT_LOPROC",
            13 => "SHT_HIPROC",
            14 => "SHT_LOUSER",
            15 => "SHT_HIUSER",
            _ => "unknown type",
        }
    }

    fn show(&self, id: usize) {
        println!("============== section header {}==============", id + 1);
        println!("sh_name:\t{}", self.sh_name);
        println!("sh_type:\t{}", self.type_to_str());
        println!("sh_flags:\t{}", self.sh_flags);
        println!("sh_addr:\t0x{:x}", self.sh_addr);
        println!("sh_offset:\t0x{:x}", self.sh_offset);
        println!("sh_size:\t{}", self.sh_size);
        println!("sh_link:\t{}", self.sh_link);
        println!("sh_info:\t{}", self.sh_info);
        println!("sh_addralign:\t{}", self.sh_addralign);
        println!("sh_entsize:\t{}", self.sh_entsize);
    }

    fn dump(&self, mmap: &[u8]) {
        const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
        const EXAMPLE_CODE_BITNESS: u32 = 64;
        const EXAMPLE_CODE_RIP: u64 = 0x0000_0000_8000_0000;
        if self.sh_flags >> 2 & 1 == 1 {
            let bytes = &mmap[self.sh_offset as usize..(self.sh_offset + self.sh_size) as usize];
            let mut decoder = Decoder::with_ip(
                EXAMPLE_CODE_BITNESS,
                bytes,
                EXAMPLE_CODE_RIP,
                DecoderOptions::NONE,
            );
            let mut formatter = NasmFormatter::new();

            formatter.options_mut().set_digit_separator("`");
            formatter.options_mut().set_first_operand_char_index(10);

            let mut output = String::new();

            let mut instruction = Instruction::default();
            while decoder.can_decode() {
                decoder.decode_out(&mut instruction);

                output.clear();
                formatter.format(&instruction, &mut output);

                print!("{:016X} ", instruction.ip());
                let start_index = (instruction.ip() - EXAMPLE_CODE_RIP) as usize;
                let instr_bytes = &bytes[start_index..start_index + instruction.len()];
                for b in instr_bytes.iter() {
                    print!("{b:02X}");
                }
                if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
                    for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                        print!("  ");
                    }
                }
                println!(" {output}");
            }
        }
    }

    fn inst_analysis(&self, inst_list: &mut HashMap<String, u32>, mmap: &[u8]) {
        const EXAMPLE_CODE_BITNESS: u32 = 64;
        let start_addr: u64 = self.sh_addr as u64;
        if self.sh_flags >> 2 & 1 == 1 {
            let bytes = &mmap[self.sh_offset as usize..(self.sh_offset + self.sh_size) as usize];
            let mut decoder = Decoder::with_ip(
                EXAMPLE_CODE_BITNESS,
                bytes,
                start_addr,
                DecoderOptions::NONE,
            );
            let mut formatter = NasmFormatter::new();

            formatter.options_mut().set_digit_separator("_");
            formatter.options_mut().set_first_operand_char_index(10);

            let mut instruction = Instruction::default();
            while decoder.can_decode() {
                decoder.decode_out(&mut instruction);
                println!("count: {:?} ", instruction.mnemonic());
                *inst_list
                    .entry(format!("{:?}", instruction.mnemonic()))
                    .or_insert(0) += 1;
            }
        }
    }
}
