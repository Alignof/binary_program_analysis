mod elf_header;
mod program_header;
mod section_header;

use elf_header::ElfHeader;
use memmap::Mmap;
use program_header::ProgramHeader;
use section_header::SectionHeader;

pub struct ElfLoader {
    pub elf_header: ElfHeader,
    pub prog_headers: Vec<ProgramHeader>,
    pub sect_headers: Vec<SectionHeader>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    pub fn new(mapped_data: Mmap) -> ElfLoader {
        let new_elf = ElfHeader::new(&mapped_data);
        let new_prog = ProgramHeader::new(&mapped_data, &new_elf);
        let new_sect = SectionHeader::new(&mapped_data, &new_elf);

        ElfLoader {
            elf_header: new_elf,
            prog_headers: new_prog,
            sect_headers: new_sect,
            mem_data: mapped_data,
        }
    }

    pub fn header_show(&self) {
        self.elf_header.show();
    }

    pub fn show_all_header(&self) {
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

    pub fn dump_segment(&self) {
        for (id, prog) in self.prog_headers.iter().enumerate() {
            prog.show(id);
            prog.segment_dump(&self.mem_data);
            println!("\n\n");
        }
    }

    pub fn dump_section(&self) {
        for (id, sect) in self.sect_headers.iter().enumerate() {
            sect.show(id);
            sect.section_dump(&self.mem_data);
            println!("\n\n");
        }
    }
}
