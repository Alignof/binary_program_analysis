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
        const STEP: u8 = 6;
        match hex {
            Some(hex) => {
                let step_up = |start: u8| (*hex - start).checked_mul(STEP).unwrap_or(255);
                let step_down = |start: u8| {
                    255_u8
                        .checked_sub((*hex - start).checked_mul(STEP).unwrap_or(255))
                        .unwrap_or(0)
                };
                let red = match *hex {
                    0..=127 => 0,
                    128..=169 => step_up(128),
                    170..=255 => 255,
                } as u8;
                let green = match *hex {
                    0..=41 => 0,
                    42..=83 => step_up(42),
                    84..=169 => 255,
                    170..=211 => step_down(170),
                    212..=255 => step_up(212),
                } as u8;
                let blue = match *hex {
                    0..=41 => step_up(0),
                    42..=83 => 255,
                    84..=127 => step_down(84),
                    128..=211 => 0,
                    212..=255 => step_up(212),
                } as u8;
                print!("{}", format!("{hex:02x}").on_truecolor(red, green, blue))
            }
            None => print!("  "),
        }
    }

    fn hex_to_ascii(&self, hex: Option<&u8>) -> ColoredString {
        match hex {
            Some(hex) => match hex {
                0 => "⋄".truecolor(125, 125, 125),
                1..=31 => "•".blue(),
                32 => "␣".truecolor(0, 80, 255),
                33..=126 => std::str::from_utf8(&[*hex]).unwrap().truecolor(0, 240, 200),
                127 => "•".blue(),
                _ => "×".yellow(),
            },
            None => " ".normal(),
        }
    }

    fn print_ascii(&self, ascii: Vec<Option<u8>>) {
        ascii.iter().enumerate().for_each(|(i, c)| {
            if i == 8 {
                self.print_delimiter()
            }
            print!("{}", self.hex_to_ascii(c.as_ref()));
        });
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
            self.print_ascii(ascii);

            self.print_end();
        }
    }

    pub fn print_footer(&self) {
        println!("└────────┴────────────────┴────────────────┴────────┴────────┘");
    }
}
