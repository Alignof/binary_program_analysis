use crate::pe::{get_u32, get_u64};

pub struct MsDosHeader {
    e_magic: u32,      // Magic number
    e_cblp: u32,       // Bytes on last page of file
    e_cp: u32,         // Pages in file
    e_crlc: u32,       // Relocations
    e_cparhdr: u32,    // Size of header in paragraphs
    e_minalloc: u32,   // Minimum extra paragraphs needed
    e_maxalloc: u32,   // Maximum extra paragraphs needed
    e_ss: u32,         // Initial (relative) SS value
    e_sp: u32,         // Initial SP value
    e_csum: u32,       // Checksum
    e_ip: u32,         // Initial IP value
    e_cs: u32,         // Initial (relative) CS value
    e_lfarlc: u32,     // File address of relocation table
    e_ovno: u32,       // Overlay number
    e_res: [u32; 4],   // Reserved words
    e_oemid: u32,      // OEM identifier (for e_oeminfo)
    e_oeminfo: u32,    // OEM information; e_oemid specific
    e_res2: [u32; 10], // Reserved words
    e_lfanew: u64,     // File address of new exe header
}

impl MsDosHeader {
    pub fn new(mmap: &[u8]) -> MsDosHeader {
        const HEADER_START: usize = 0;
        MsDosHeader {
            e_magic: get_u32(mmap, HEADER_START + 0),
            e_cblp: get_u32(mmap, HEADER_START + 4),
            e_cp: get_u32(mmap, HEADER_START + 12),
            e_crlc: get_u32(mmap, HEADER_START + 16),
            e_cparhdr: get_u32(mmap, HEADER_START + 20),
            e_minalloc: get_u32(mmap, HEADER_START + 24),
            e_maxalloc: get_u32(mmap, HEADER_START + 28),
            e_ss: get_u32(mmap, HEADER_START + 32),
            e_sp: get_u32(mmap, HEADER_START + 36),
            e_csum: get_u32(mmap, HEADER_START + 40),
            e_ip: get_u32(mmap, HEADER_START + 44),
            e_cs: get_u32(mmap, HEADER_START + 48),
            e_lfarlc: get_u32(mmap, HEADER_START + 52),
            e_ovno: get_u32(mmap, HEADER_START + 56),
            e_res: [0; 4],
            e_oemid: get_u32(mmap, HEADER_START + 72),
            e_oeminfo: get_u32(mmap, HEADER_START + 76),
            e_res2: [0; 10],
            e_lfanew: get_u64(mmap, HEADER_START + 116),
        }
    }

    pub fn show(&self) {
        println!("================ msdos header ================");
        println!("e_magic:\t{:#x}", self.e_magic);
        println!("e_cblp:\t\t{:#x}", self.e_cblp);
        println!("e_cp:\t\t{:#x}", self.e_cp);
        println!("e_crlc:\t\t{:#x}", self.e_crlc);
        println!("e_cparhdr:\t{}", self.e_cparhdr);
        println!("e_minalloc:\t{:#x}", self.e_minalloc);
        println!("e_maxalloc:\t{}", self.e_maxalloc);
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
}
