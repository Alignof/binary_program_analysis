pub mod elf;
mod elf_test;
mod function;
pub mod pe;
mod pe_test;

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
}
