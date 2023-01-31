#[cfg(test)]
mod tests {
    use crate::{loader, visualize};
    use memmap::Mmap;
    use std::fs::File;

    #[test]
    fn elf_32_test() -> std::io::Result<()> {
        let filename = "./test/Elf32";
        let file = File::open(filename)?;
        let mapped_data = unsafe { Mmap::map(&file)? };
        let loader = loader::elf::ElfLoader::new(mapped_data);

        loader.header_show();
        loader.show_segment();
        loader.show_section();
        loader.disassemble();
        visualize::dump(loader.mem_data());

        Ok(())
    }

    #[test]
    fn elf_64_test() -> std::io::Result<()> {
        let filename = "./test/Elf64";
        let file = File::open(filename)?;
        let mapped_data = unsafe { Mmap::map(&file)? };
        let loader = loader::elf::ElfLoader::new(mapped_data);

        loader.header_show();
        loader.show_segment();
        loader.show_section();
        loader.disassemble();
        visualize::dump(loader.mem_data());

        Ok(())
    }
}
