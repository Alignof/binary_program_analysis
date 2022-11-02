use crate::pe::{get_u16, get_u32};

#[derive(Debug)]
pub struct MsDosHeader {
    e_magic: u16,      // Magic number
    e_cblp: u16,       // Bytes on last page of file
    e_cp: u16,         // Pages in file
    e_crlc: u16,       // Relocations
    e_cparhdr: u16,    // Size of header in paragraphs
    e_minalloc: u16,   // Minimum extra paragraphs needed
    e_maxalloc: u16,   // Maximum extra paragraphs needed
    e_ss: u16,         // Initial (relative) SS value
    e_sp: u16,         // Initial SP value
    e_csum: u16,       // Checksum
    e_ip: u16,         // Initial IP value
    e_cs: u16,         // Initial (relative) CS value
    e_lfarlc: u16,     // File address of relocation table
    e_ovno: u16,       // Overlay number
    e_res: [u16; 4],   // Reserved words
    e_oemid: u16,      // OEM identifier (for e_oeminfo)
    e_oeminfo: u16,    // OEM information; e_oemid specific
    e_res2: [u16; 10], // Reserved words
    e_lfanew: u32,     // File address of new exe header
}

impl MsDosHeader {
    pub fn new(mmap: &[u8]) -> MsDosHeader {
        const HEADER_START: usize = 0;
        MsDosHeader {
            e_magic: get_u16(mmap, HEADER_START + 0),
            e_cblp: get_u16(mmap, HEADER_START + 2),
            e_cp: get_u16(mmap, HEADER_START + 4),
            e_crlc: get_u16(mmap, HEADER_START + 6),
            e_cparhdr: get_u16(mmap, HEADER_START + 8),
            e_minalloc: get_u16(mmap, HEADER_START + 10),
            e_maxalloc: get_u16(mmap, HEADER_START + 12),
            e_ss: get_u16(mmap, HEADER_START + 14),
            e_sp: get_u16(mmap, HEADER_START + 16),
            e_csum: get_u16(mmap, HEADER_START + 18),
            e_ip: get_u16(mmap, HEADER_START + 20),
            e_cs: get_u16(mmap, HEADER_START + 22),
            e_lfarlc: get_u16(mmap, HEADER_START + 24),
            e_ovno: get_u16(mmap, HEADER_START + 26),
            e_res: [0; 4],
            e_oemid: get_u16(mmap, HEADER_START + 34),
            e_oeminfo: get_u16(mmap, HEADER_START + 36),
            e_res2: [0; 10],
            e_lfanew: get_u32(mmap, HEADER_START + 60),
        }
    }

    pub fn show(&self) {
        println!("\n================ msdos header ================");
        println!("e_magic:\t{:#x}", self.e_magic);
        println!("e_cblp:\t\t{:#x}", self.e_cblp);
        println!("e_cp:\t\t{:#x}", self.e_cp);
        println!("e_crlc:\t\t{:#x}", self.e_crlc);
        println!("e_cparhdr:\t{}", self.e_cparhdr);
        println!("e_minalloc:\t{:#x}", self.e_minalloc);
        println!("e_maxalloc:\t{:#x}", self.e_maxalloc);
        println!("e_ss:\t\t{}", self.e_ss);
        println!("e_sp:\t\t{}", self.e_sp);
        println!("e_csum:\t\t{}", self.e_csum);
        println!("e_ip:\t\t{}", self.e_ip);
        println!("e_cs:\t\t{}", self.e_cs);
        println!("e_lfarlc:\t{}", self.e_lfarlc);
        println!("e_ovno:\t\t{}", self.e_ovno);
        println!("e_res:\t\t{:?}", self.e_res);
        println!("e_oemid:\t{}", self.e_oemid);
        println!("e_oeminfo:\t{}", self.e_oeminfo);
        println!("e_res2:\t\t{:?}", self.e_res2);
        println!("e_lfanew:\t{:#x}", self.e_lfanew);
    }

    pub fn nt_offset(&self) -> usize {
        self.e_lfanew as usize
    }
}
