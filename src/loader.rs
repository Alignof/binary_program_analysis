pub mod elf;
mod elf_test;
mod function;
pub mod pe;
mod pe_test;

use std::collections::HashMap;

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

pub enum Arch {
    Bit32,
    Bit64,
}

pub trait Loader {
    fn mem_data(&self) -> &[u8];
    fn header_show(&self);
    fn show_segment(&self);
    fn show_section(&self);
    fn disassemble(&self);
    fn show_all_header(&self);
    fn analysis(&self);
    fn byte_histogram(&self) {
        use plotters::prelude::*;

        let mut histogram = (0..255)
            .collect::<Vec<u8>>()
            .iter()
            .map(|x| (*x, 0_u32))
            .collect::<HashMap<u8, u32>>();

        for m in self.mem_data().iter() {
            *histogram.entry(*m).or_insert(0) += 1;
        }
        let max_count: u32 = *histogram.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;

        // calc entropy
        let mut entropy: f64 = 0.0;
        for (_, count) in histogram.iter() {
            let p: f64 = (*count as f64) / (self.mem_data().len() as f64);
            if p != 0.0 {
                entropy -= p * p.log(2.0);
            }
        }
        println!("entropy: {}", entropy);

        let root = BitMapBackend::new("target/histogram.png", (1080, 720)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .margin(10)
            .caption("Byte histogram", ("sans-serif", 25.0))
            .build_cartesian_2d((0u32..255u32).into_segmented(), 0u32..max_count)
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(BLACK.mix(0.5))
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
    }
}
