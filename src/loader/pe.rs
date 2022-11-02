mod msdos_header;
mod nt_headers;
mod section_header;

use memmap::Mmap;
use msdos_header::MsDosHeader;
use nt_headers::NtHeader;
use section_header::SectionHeader;

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
