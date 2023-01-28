#[cfg(test)]
mod tests {
    use crate::loader;
    use memmap::Mmap;
    use std::fs::File;

    #[test]
    fn pe_32_test() -> std::io::Result<()> {
        let filename = "./test/Pe32.exe";
        let file = File::open(filename)?;
        let mapped_data = unsafe { Mmap::map(&file)? };
        let loader = loader::pe::PeLoader::new(mapped_data);
        loader.header_show();
        Ok(())
    }

    #[test]
    fn pe_64_test() -> std::io::Result<()> {
        let filename = "./test/Pe64.exe";
        let file = File::open(filename)?;
        let mapped_data = unsafe { Mmap::map(&file)? };
        let loader = loader::pe::PeLoader::new(mapped_data);
        loader.header_show();
        Ok(())
    }
}
