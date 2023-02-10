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
        let other_filename = "./test/Elf64";
        let other_file = File::open(other_filename)?;
        let other = unsafe { Mmap::map(&other_file)? };

        visualize::dump(&mapped_data);
        visualize::diff(&mapped_data, &other);

        let loader = loader::elf::ElfLoader::new(mapped_data);
        loader.header_show();
        loader.show_segment();
        loader.show_section();
        loader.disassemble();
        loader.analysis();

        Ok(())
    }

    #[test]
    fn elf_64_test() -> std::io::Result<()> {
        let filename = "./test/Elf64";
        let file = File::open(filename)?;
        let mapped_data = unsafe { Mmap::map(&file)? };
        let other_filename = "./test/Elf32";
        let other_file = File::open(other_filename)?;
        let other = unsafe { Mmap::map(&other_file)? };

        visualize::dump(&mapped_data);
        visualize::diff(&mapped_data, &other);

        let loader = loader::elf::ElfLoader::new(mapped_data);
        loader.header_show();
        loader.show_segment();
        loader.show_section();
        loader.disassemble();
        loader.analysis();

        Ok(())
    }
}
