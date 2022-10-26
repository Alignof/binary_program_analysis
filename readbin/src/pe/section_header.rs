use crate::pe::{get_u16, get_u32, get_u64};

pub struct SectionHeader {
    name: String,
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_linenumbers: u32,
    number_of_relocations: u16,
    number_of_linenumbers: u16,
    characteristics: u32,
}


impl SectionHeader {
    pub fn new(mmap: &[u8], sect_num: usize, header_start: usize) -> Vec<SectionHeader> {
        const SECT_SIZE: usize = 40;
        let mut section_headers = Vec::new();
        for offset in (header_start .. sect_num * SECT_SIZE).step_by(SECT_SIZE) {
            section_headers.push(
                SectionHeader {
                    name: get_u64(mmap, offset)
                        .to_le_bytes()
                        .iter()
                        .map(|b| *b as char)
                        .collect::<String>(),
                    virtual_size: get_u32(mmap, offset + 8),
                    virtual_address: get_u32(mmap, offset + 12),
                    size_of_raw_data: get_u32(mmap, offset + 16),
                    pointer_to_raw_data: get_u32(mmap, offset + 20),
                    pointer_to_relocations: get_u32(mmap, offset + 24),
                    pointer_to_linenumbers: get_u32(mmap, offset + 28),
                    number_of_relocations: get_u16(mmap, offset + 32),
                    number_of_linenumbers: get_u16(mmap, offset + 34),
                    characteristics: get_u32(mmap, offset + 36),
                }
            )
        }

        section_headers
    }
    pub fn show(&self) {
        println!("--- section ---");
        println!("name:\t{}", self.name);
        println!("virtual_size:\t{:#x}", self.virtual_size);
        println!("virtual_address:\t{:#x}", self.virtual_address);
        println!("size_of_raw_data:\t{:#x}", self.size_of_raw_data);
        println!("pointer_to_raw_data:\t{:#x}", self.pointer_to_raw_data);
        println!("pointer_to_relocations:\t{:#x}", self.pointer_to_relocations);
        println!("pointer_to_linenumbers:\t{:#x}", self.pointer_to_linenumbers);
        println!("number_of_relocations:\t{:#x}", self.number_of_relocations);
        println!("number_of_linenumbers:\t{:#x}", self.number_of_linenumbers);
        println!("characteristics:\t{:#x}", self.characteristics);
    }

    pub fn dump(&self, mmap: &[u8]) {
        println!("--------------------------------");
        let mut dump_head = self.pointer_to_raw_data;
        while dump_head < self.pointer_to_raw_data + self.size_of_raw_data {
            let mdump = get_u32(mmap, dump_head as usize);
            dump_head += 4;

            print!("{:<08x} ", mdump);

            if dump_head % 32 == 0 {
                println!();
            }
        }
    }
}
