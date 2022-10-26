use crate::pe::{get_u16, get_u32, get_u64};
pub struct NtHeader {
    offset: usize,
    signature: u32,
    file_header: FileHeader,
    optional_header: OptionalHeader,
}

struct FileHeader {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symtab: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

struct OptionalHeader {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: u32,

    image_base: u32,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u32,
    minor_operating_system_version: u32,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    _data_directory: u32,
}

impl NtHeader {
    pub fn new(mmap: &[u8], offset: usize) -> NtHeader {
        let file_offset = offset + 4;
        let optional_offset = file_offset + 20;
        NtHeader {
            offset,
            signature: get_u32(mmap, offset),
            file_header: FileHeader::new(mmap, file_offset),
            optional_header: OptionalHeader::new(mmap, optional_offset),
        }
    }

    pub fn sect_off(&self) -> usize {
        const MAGIC_SIZE: usize = 4;
        const FILEHEADER_SIZE: usize = 20;
        self.offset + MAGIC_SIZE + FILEHEADER_SIZE + self.file_header.size_of_optional_header as usize
    }

    pub fn sect_num(&self) -> usize {
        self.file_header.number_of_sections as usize
    }

    pub fn show(&self) {
        println!("\n=== NtHeader ===");
        println!("signature: {:#x}", self.signature);
        self.file_header.show();
        self.optional_header.show();
    }
}

impl FileHeader {
    pub fn new(mmap: &[u8], offset: usize) -> FileHeader {
        FileHeader {
            machine: get_u16(mmap, offset),
            number_of_sections: get_u16(mmap, offset + 2),
            time_date_stamp: get_u32(mmap, offset + 4),
            pointer_to_symtab: get_u32(mmap, offset + 8),
            number_of_symbols: get_u32(mmap, offset + 12),
            size_of_optional_header: get_u16(mmap, offset + 16),
            characteristics: get_u16(mmap, offset + 18),
        }
    }

    pub fn show(&self) {
        println!("--- FileHeader ---");
        println!("machine:\t{:#x}", self.machine);
        println!("number_of_sections:\t{:#x}", self.number_of_sections);
        println!("time_date_stamp:\t{:#x}", self.time_date_stamp);
        println!("pointer_to_symtab:\t{:#x}", self.pointer_to_symtab);
        println!("number_of_symbols:\t{:#x}", self.number_of_symbols);
        println!("size_of_optional_header:\t{:#x}", self.size_of_optional_header);
        println!("characteristics:\t{:#x}", self.characteristics);
    }
}


impl OptionalHeader {
    pub fn new(mmap: &[u8], offset: usize) -> OptionalHeader {
        OptionalHeader {
            magic: get_u16(mmap, offset),
            major_linker_version: mmap[offset + 2],
            minor_linker_version: mmap[offset + 3],
            size_of_code: get_u32(mmap, offset + 4),
            size_of_initialized_data: get_u32(mmap, offset + 8),
            size_of_uninitialized_data: get_u32(mmap, offset + 12),
            address_of_entry_point: get_u32(mmap, offset + 16),
            base_of_code: get_u32(mmap, offset + 20),
            base_of_data: get_u32(mmap, offset + 24),
            image_base: get_u32(mmap, offset + 28),
            section_alignment: get_u32(mmap, offset + 32),
            file_alignment: get_u32(mmap, offset + 36),
            major_operating_system_version: get_u32(mmap, offset + 40),
            minor_operating_system_version: get_u32(mmap, offset + 42),
            major_image_version: get_u16(mmap, offset + 44),
            minor_image_version: get_u16(mmap, offset + 46),
            major_subsystem_version: get_u16(mmap, offset + 48),
            minor_subsystem_version: get_u16(mmap, offset + 50),
            win32_version_value: get_u32(mmap, offset + 52),
            size_of_image: get_u32(mmap, offset + 56),
            size_of_headers: get_u32(mmap, offset + 60),
            check_sum: get_u32(mmap, offset + 64),
            subsystem: get_u16(mmap, offset + 68),
            dll_characteristics: get_u16(mmap, offset + 70),
            size_of_stack_reserve: get_u64(mmap, offset + 72),
            size_of_stack_commit: get_u64(mmap, offset + 80),
            size_of_heap_reserve: get_u64(mmap, offset + 88),
            size_of_heap_commit: get_u64(mmap, offset + 96),
            loader_flags: get_u32(mmap, offset + 104),
            number_of_rva_and_sizes: get_u32(mmap, offset + 108),
            _data_directory: get_u32(mmap, offset + 112),
        }
    }

    pub fn show(&self) {
        println!("--- OptionalHeader ---");
        println!("magic:\t{:#x}", self.magic);
        println!("major_linker_version:\t{:#x}", self.major_linker_version);
        println!("minor_linker_version:\t{:#x}", self.minor_linker_version);
        println!("size_of_code:\t{:#x}", self.size_of_code);
        println!("size_of_initialized_data:\t{:#x}", self.size_of_initialized_data);
        println!("size_of_uninitialized_data:\t{:#x}", self.size_of_uninitialized_data);
        println!("address_of_entry_point:\t{:#x}", self.address_of_entry_point);
        println!("base_of_code:\t{:#x}", self.base_of_code);
        println!("base_of_data:\t{:#x}", self.base_of_data);

        println!("image_base:\t{:#x}", self.image_base);
        println!("section_alignment:\t{:#x}", self.section_alignment);
        println!("file_alignment:\t{:#x}", self.file_alignment);
        println!("major_operating_system_version:\t{:#x}", self.major_operating_system_version);
        println!("minor_operating_system_version:\t{:#x}", self.minor_operating_system_version);
        println!("major_image_version:\t{:#x}", self.major_image_version);
        println!("minor_image_version:\t{:#x}", self.minor_image_version);
        println!("major_subsystem_version:\t{:#x}", self.major_subsystem_version);
        println!("minor_subsystem_version:\t{:#x}", self.minor_subsystem_version);
        println!("win32_version_value:\t{:#x}", self.win32_version_value);
        println!("size_of_image:\t{:#x}", self.size_of_image);
        println!("size_of_headers:\t{:#x}", self.size_of_headers);
        println!("check_sum:\t{:#x}", self.check_sum);
        println!("subsystem:\t{:#x}", self.subsystem);
        println!("dll_characteristics:\t{:#x}", self.dll_characteristics);
        println!("size_of_stack_reserve:\t{:#x}", self.size_of_stack_reserve);
        println!("size_of_stack_commit:\t{:#x}", self.size_of_stack_commit);
        println!("size_of_heap_reserve:\t{:#x}", self.size_of_heap_reserve);
        println!("size_of_heap_commit:\t{:#x}", self.size_of_heap_commit);
        println!("loader_flags:\t{:#x}", self.loader_flags);
        println!("number_of_rva_and_sizes:\t{:#x}", self.number_of_rva_and_sizes);
        println!("_data_directory:\t{:#x}", self._data_directory);
    }
}
