use colored::{ColoredString, Colorize};

pub struct HexDump<'a> {
    mem_data: &'a [u8],
}

impl<'a> HexDump<'_> {
    pub fn new(mem_data: &'a [u8]) -> HexDump<'a> {
        HexDump { mem_data }
    }

    pub fn print_header(&self) {
        println!("┌────────┬────────────────┬────────────────┬────────┬────────┐");
    }

    fn print_row_number(&self, row: usize) {
        print!("│{row:08x}│");
    }

    fn print_delimiter(&self) {
        print!("┊");
    }

    fn print_end(&self) {
        println!("│");
    }

    fn print_hex(&self, hex: Option<&u8>) {
        match hex {
            Some(hex) => {
                let color = *hex as u32;
                let red = (((color >> 4) & 0xf) * 0xff / 0xf) as u8;
                let green = (((color >> 2) & 0xf) * 0xff / 0xf) as u8;
                let blue = ((color & 0xf) * 0xff / 0xf) as u8;
                print!("{}", format!("{hex:02x}").on_truecolor(red, green, blue))
            }
            None => print!("  "),
        }
    }

    fn hex_to_ascii(&self, hex: Option<&u8>) -> ColoredString {
        match hex {
            Some(hex) => match hex {
                0 => "⋄".truecolor(125, 125, 125),
                1..=31 => "•".green(),
                32 => " ".normal(),
                33..=126 => std::str::from_utf8(&[*hex]).unwrap().bright_blue(),
                127 => "•".green(),
                _ => "×".yellow(),
            },
            None => " ".normal(),
        }
    }

    pub fn print_data(&self) {
        for (row, chunk) in self.mem_data.chunks(16).enumerate() {
            self.print_row_number(row << 4);
            let mut ascii: Vec<Option<u8>> = Vec::new();
            for index in 0..16 {
                if index == 8 {
                    self.print_delimiter();
                }
                self.print_hex(chunk.get(index));
                ascii.push(chunk.get(index).copied());
            }

            print!("│");

            ascii.iter().enumerate().for_each(|(i, c)| {
                if i == 8 {
                    self.print_delimiter()
                }
                print!("{}", self.hex_to_ascii(c.as_ref()));
            });
            self.print_end();
        }
    }

    pub fn print_footer(&self) {
        println!("└────────┴────────────────┴────────────────┴────────┴────────┘");
    }
}