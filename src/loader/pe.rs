mod msdos_header;
mod nt_headers;
mod section_header;

use crate::loader::{get_u32, get_u64, Function, Loader};
use memmap::Mmap;
use msdos_header::MsDosHeader;
use nt_headers::NtHeader;
use section_header::SectionHeader;
use std::collections::HashMap;

pub struct PeLoader {
    pub msdos_header: MsDosHeader,
    pub nt_headers: NtHeader,
    pub sect_headers: Vec<SectionHeader>,
    pub functions: Vec<Function>,
    pub mem_data: Mmap,
}

impl PeLoader {
    fn create_func_table(mmap: &[u8], sect_headers: &[SectionHeader]) -> Vec<Function> {
        let symtab = sect_headers.iter().find(|s| s.name == ".symtab");
        let strtab = sect_headers.iter().find(|s| s.name == ".strtab");

        const ST_SIZE: usize = 32;
        let mut functions: Vec<Function> = Vec::new();
        if let (Some(symtab), Some(strtab)) = (symtab, strtab) {
            for symtab_off in (symtab.pointer_to_raw_data
                ..symtab.pointer_to_raw_data + symtab.size_of_raw_data)
                .step_by(ST_SIZE)
            {
                let st_name_off = get_u32(mmap, symtab_off as usize);
                let st_name = mmap[(strtab.pointer_to_raw_data + st_name_off) as usize..]
                    .iter()
                    .take_while(|c| **c as char != '\0')
                    .map(|c| *c as char)
                    .collect::<String>();
                let st_addr = get_u64(mmap, (symtab_off + 16) as usize);
                let st_size = get_u64(mmap, (symtab_off + 24) as usize);

                functions.push(Function {
                    name: st_name,
                    addr: st_addr,
                    size: st_size,
                });
            }
        }

        functions
    }

    pub fn new(mapped_data: Mmap) -> Box<dyn Loader> {
        let new_msdos = MsDosHeader::new(&mapped_data);
        let new_nt = NtHeader::new(&mapped_data, new_msdos.nt_offset());
        let new_sect = SectionHeader::new(&mapped_data, new_nt.sect_num(), new_nt.sect_off());
        let new_func = Self::create_func_table(&mapped_data, &new_sect);

        Box::new(PeLoader {
            msdos_header: new_msdos,
            nt_headers: new_nt,
            sect_headers: new_sect,
            functions: new_func,
            mem_data: mapped_data,
        })
    }
}

impl Loader for PeLoader {
    fn header_show(&self) {
        self.msdos_header.show();
    }

    fn show_segment(&self) {
        self.show_section();
    }

    fn show_section(&self) {
        for sect in &self.sect_headers {
            sect.show();
        }
    }

    fn disassemble(&self) {
        for sect in &self.sect_headers {
            sect.show();
            sect.dump(&self.mem_data);
        }
    }

    fn show_all_header(&self) {
        self.msdos_header.show();
        self.nt_headers.show();
        for sect in &self.sect_headers {
            sect.show();
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
