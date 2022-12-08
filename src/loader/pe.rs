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

    fn byte_histogram(&self) {
        use plotters::prelude::*;

        let mut histogram = (0..255)
            .collect::<Vec<u8>>()
            .iter()
            .map(|x| (*x, 0_u32))
            .collect::<HashMap<u8, u32>>();

        for m in self.mem_data.iter() {
            *histogram.entry(*m).or_insert(0) += 1;
        }
        let max_count: u32 = *histogram.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

        let root = BitMapBackend::new("target/histogram.png", (1080, 720)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(10)
            .caption("Byte histogram", ("sans-serif", 25.0))
            .build_cartesian_2d((0u32..255u32).into_segmented(), 0u32..max_count)
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&BLACK.mix(0.5))
            .y_desc("Count")
            .x_desc("Byte")
            .axis_desc_style(("sans-serif", 15))
            .draw()
            .unwrap();

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(RED.filled())
                    .margin(0)
                    .data(histogram.iter().map(|(x, y)| (*x as u32, *y))),
            )
            .unwrap();

        if let Some(zero_count) = histogram.get(&0) {
            if zero_count > &max_count {
                root.draw_text(
                    &format!("↓{}", zero_count),
                    &TextStyle::from(("sans-serif", 13.0).into_font()),
                    (47, 30),
                )
                .unwrap();
                root.draw_text(
                    "≈",
                    &TextStyle::from(("sans-serif", 20.0).into_font()),
                    (45, 45),
                )
                .unwrap();
            }
        }
    }
}
