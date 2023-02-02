mod hex_dump;

use hex_dump::HexDump;
use std::collections::HashMap;

fn hex_to_color(hex: u8) -> (u8, u8, u8) {
    const STEP: u8 = 6;
    let step_up = |start: u8| (hex - start).saturating_mul(STEP);
    let step_down = |start: u8| 255_u8.saturating_sub((hex - start).saturating_mul(STEP));
    let red = match hex {
        0..=127 => 0,
        128..=169 => step_up(128),
        170..=255 => 255,
    };
    let green = match hex {
        0..=41 => 0,
        42..=83 => step_up(42),
        84..=169 => 255,
        170..=211 => step_down(170),
        212..=255 => step_up(212),
    };
    let blue = match hex {
        0..=41 => step_up(0),
        42..=83 => 255,
        84..=127 => step_down(84),
        128..=211 => 0,
        212..=255 => step_up(212),
    };
    (red, green, blue)
}

pub fn dump(mem_data: &[u8]) {
    let hex_dump = HexDump::new(mem_data);
    hex_dump.print_header();
    hex_dump.print_data();
    hex_dump.print_footer();
}

pub fn diff(mem_data: &[u8], other: &[u8]) {
    let hex_dump = HexDump::new(mem_data);
    hex_dump.print_header();
    hex_dump.print_diff(other);
    hex_dump.print_footer();
}

pub fn create_byte_histogram(mem_data: &[u8]) {
    use colored::Colorize;

    let mut histogram = (0..=255)
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
    println!("entropy: {entropy}");

    const BAR: [&str; 8] = ["", "▏", "▎", "▍", "▌", "▋	", "▊", "▉"];
    let mut histogram: Vec<(&u8, &u32)> = histogram.iter().collect();
    histogram.sort_by(|a, b| (a.0).cmp(b.0));
    for (hex, count) in histogram.iter() {
        print!("{hex:02x}: ");
        if **count < 512 {
            (0..=(*count / 8)).for_each(|_| print!("█"));
            println!("{} {count}", BAR[(*count % 8) as usize]);
        } else {
            (0..=67).for_each(|_| print!("█"));
            println!("▓▒░ {count}");
        }
    }
    for (hex, count) in histogram.iter() {
        let parcent = **count as f64 / max_count as f64;
        let count = (parcent * 255.0) as u8;
        let (r, g, b) = hex_to_color(count);
        print!("{}", format!("{hex:02x}").on_truecolor(r, g, b));

        if *hex % 16 == 15 {
            println!();
        }
    }
    println!();
}
