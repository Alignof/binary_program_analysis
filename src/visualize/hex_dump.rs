pub struct HexDump<'a> {
    mem_data: &'a [u8],
}

impl<'a> HexDump<'_> {
    pub fn new(mem_data: &'a [u8]) -> HexDump<'a> {
        HexDump { mem_data }
    }

    pub fn print_header(&self) {
        println!("┌────────┬─────────────────────────┬─────────────────────────┐");
    }

    fn print_row_number(&self, row: usize) {
        print!("│{row:08x}│ ");
    }

    fn print_delimiter(&self) {
        print!("┊ ");
    }

    fn print_end(&self) {
        println!("│");
    }

    fn print_hex(&self, hex: Option<&u8>) {
        match hex {
            Some(hex) => print!("{hex:02x} "),
            None => print!("   "),
        }
    }

    pub fn print_data(&self) {
        for (row, chunk) in self.mem_data.chunks(16).enumerate() {
            self.print_row_number(row << 4);
            for index in 0..16 {
                if index == 8 {
                    self.print_delimiter();
                }
                self.print_hex(chunk.get(index));
            }
            self.print_end();
        }
    }

    pub fn print_footer(&self) {
        println!("└────────┴─────────────────────────┴─────────────────────────┘");
    }
}
