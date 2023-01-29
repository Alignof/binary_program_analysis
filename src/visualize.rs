use std::collections::HashMap;

pub fn dump(mem_data: &[u8]) {
    for m in mem_data.iter() {
        print!("{m:02x} ")
    }
}

pub fn create_byte_histogram(mem_data: &[u8]) {
    use plotters::prelude::*;

    let mut histogram = (0..255)
        .collect::<Vec<u8>>()
        .iter()
        .map(|x| (*x, 0_u32))
        .collect::<HashMap<u8, u32>>();

    for m in mem_data.iter() {
        *histogram.entry(*m).or_insert(0) += 1;
    }
    let max_count: u32 = *histogram.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;

    // calc entropy
    let mut entropy: f64 = 0.0;
    for (_, count) in histogram.iter() {
        let p: f64 = (*count as f64) / (mem_data.len() as f64);
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
