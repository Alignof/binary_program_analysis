use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub addr: u64,
    pub size: u64,
}

impl Function {
    pub fn inst_analysis(&self, inst_list: &mut HashMap<String, i32>, mmap: &[u8]) -> Vec<u64> {
        println!("<function: {}>", self.name);
        const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
        const EXAMPLE_CODE_BITNESS: u32 = 64;
        let start_addr: u64 = self.addr;
        let bytes = &mmap[self.addr as usize..(self.addr + self.size) as usize];
        let mut decoder = Decoder::with_ip(
            EXAMPLE_CODE_BITNESS,
            bytes,
            start_addr,
            DecoderOptions::NONE,
        );
        let mut formatter = NasmFormatter::new();

        formatter.options_mut().set_digit_separator("_");
        formatter.options_mut().set_first_operand_char_index(10);

        let mut output = String::new();
        let mut instruction = Instruction::default();
        let mut call_addrs = Vec::new();
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            output.clear();
            formatter.format(&instruction, &mut output);

            print!("{:016X} ", instruction.ip());
            let start_index = (instruction.ip() - start_addr) as usize;
            let instr_bytes = &bytes[start_index..start_index + instruction.len()];
            for b in instr_bytes.iter() {
                print!("{:02X}", b);
            }
            if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
                for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                    print!("  ");
                }
            }
            println!(" {}", output);

            *inst_list
                .entry(format!("{:?}", instruction.mnemonic()))
                .or_insert(0) += 1;

            if instruction.is_call_near()
                || instruction.is_call_far()
                || instruction.is_call_near_indirect()
                || instruction.is_call_far_indirect()
            {
                call_addrs.push(instruction.memory_displacement64());
            }
        }

        call_addrs
    }
}
