mod msdos_header;
mod nt_headers;
mod section_header;

use memmap::Mmap;
use msdos_header::MsDosHeader;
use nt_headers::NtHeader;
use section_header::SectionHeader;

#[allow(clippy::identity_op)]
pub fn get_u16(mmap: &[u8], index: usize) -> u16 {
    (mmap[index + 1] as u16) << 8 | (mmap[index + 0] as u16)
}

#[allow(clippy::identity_op)]
pub fn get_u32(mmap: &[u8], index: usize) -> u32 {
    (mmap[index + 3] as u32) << 24
        | (mmap[index + 2] as u32) << 16
        | (mmap[index + 1] as u32) << 8
        | (mmap[index + 0] as u32)
}

#[allow(clippy::identity_op)]
pub fn get_u64(mmap: &[u8], index: usize) -> u64 {
    (mmap[index + 7] as u64) << 56
        | (mmap[index + 6] as u64) << 48
        | (mmap[index + 5] as u64) << 40
        | (mmap[index + 4] as u64) << 32
        | (mmap[index + 3] as u64) << 24
        | (mmap[index + 2] as u64) << 16
        | (mmap[index + 1] as u64) << 8
        | (mmap[index + 0] as u64)
}

pub struct PeLoader {
    pub msdos_header: MsDosHeader,
    pub nt_headers: NtHeader,
    pub sect_headers: Vec<SectionHeader>,
    pub mem_data: Mmap,
}

impl PeLoader {
    pub fn new(mapped_data: Mmap) -> PeLoader {
        let new_msdos = MsDosHeader::new(&mapped_data);
        let new_nt = NtHeader::new(&mapped_data, new_msdos.nt_offset());
        let new_sect = SectionHeader::new(&mapped_data, new_nt.sect_num(), new_nt.sect_off());
        PeLoader {
            msdos_header: new_msdos,
            nt_headers: new_nt,
            sect_headers: new_sect,
            mem_data: mapped_data,
        }
    }

    pub fn header_show(&self) {
        self.msdos_header.show();
    }

    pub fn dump_section(&self) {
        for sect in &self.sect_headers {
            sect.show();
            sect.dump(&self.mem_data);
        }
    }

    pub fn show_all_header(&self) {
        self.msdos_header.show();
        self.nt_headers.show();
        for sect in &self.sect_headers {
            sect.show();
        }

    }
}

