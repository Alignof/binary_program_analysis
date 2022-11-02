mod loader;

use clap::{arg, AppSettings, ArgGroup};
use memmap::Mmap;
use std::fs::File;

#[allow(non_camel_case_types)]
pub enum ExeOption {
    OPT_DEFAULT,
    OPT_ELFHEAD,
    OPT_PROG,
    OPT_SECT,
    OPT_DISASEM,
    OPT_SHOWALL,
}

fn main() -> std::io::Result<()> {
    let app = clap::app_from_crate!()
        .arg(arg!(<filename> "target file path").group("ELF"))
        .arg(arg!(-e --elfhead ... "Show header"))
        .arg(arg!(-p --program ... "Show all segments"))
        .arg(arg!(-s --section ... "Show all sections"))
        .arg(arg!(-d --dump ... "Dump ELF/PE"))
        .arg(arg!(-a --all ... "Show all ELF/PE data"))
        .group(
            ArgGroup::new("run option")
                .args(&["elfhead", "dump", "program", "section", "all"])
                .required(false),
        )
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();

    let filename = match app.value_of("filename") {
        Some(f) => f.to_string(),
        None => panic!("please specify target ELF file."),
    };

    let flag_map = || {
        (
            app.is_present("elfhead"),
            app.is_present("program"),
            app.is_present("section"),
            app.is_present("dump"),
            app.is_present("all"),
        )
    };
    let exe_option = match flag_map() {
        (true, _, _, _, _) => ExeOption::OPT_ELFHEAD,
        (_, true, _, _, _) => ExeOption::OPT_PROG,
        (_, _, true, _, _) => ExeOption::OPT_SECT,
        (_, _, _, true, _) => ExeOption::OPT_DISASEM,
        (_, _, _, _, true) => ExeOption::OPT_SHOWALL,
        _ => ExeOption::OPT_DEFAULT,
    };

    let file = File::open(filename)?;
    let mapped_data = unsafe { Mmap::map(&file)? };

    const ELF_HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    const PE_HEADER_MAGIC: [u8; 2] = [0x4d, 0x5a];

    let loader = if mapped_data[0..4] == ELF_HEADER_MAGIC {
        loader::elf::ElfLoader::new(mapped_data)
    } else if mapped_data[0..2] == PE_HEADER_MAGIC {
        loader::pe::PeLoader::new(mapped_data)
    } else {
        panic!("unrecognized file format")
    };

    match exe_option {
        ExeOption::OPT_DEFAULT => loader.header_show(),
        ExeOption::OPT_ELFHEAD => loader.header_show(),
        ExeOption::OPT_PROG => loader.dump_segment(),
        ExeOption::OPT_SECT => loader.dump_section(),
        ExeOption::OPT_DISASEM => loader.dump_section(),
        ExeOption::OPT_SHOWALL => loader.show_all_header(),
    }

    Ok(())
}
