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

    pub fn print_data(&self) {
        for (row, chunk) in self
            .mem_data
            .chunks(8 * 2)
            .map(|x| x.chunks(8).collect::<Vec<_>>())
            .enumerate()
        {
            self.print_row_number(row);
            let _ = chunk[0].iter().for_each(|hex| print!("{hex:02x} "));
            self.print_delimiter();
            if chunk.len() == 2 {
                let _ = chunk[1].iter().for_each(|hex| print!("{hex:02x} "));
            } else {
                (0..8).for_each(|_| print!("   "));
            }
            self.print_end();
        }
    }

    pub fn print_footer(&self) {
        println!("└────────┴─────────────────────────┴─────────────────────────┘");
    }
}
