use std::env;
use std::fs;
use std::path;
use std::process;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use ironring::{name_hashes, repackers, unpackers};

fn main() {
    let default_namefilepath: &str = &get_default_namefilepath();
    let matches = App::new("Rusted Iron Ring")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("bhd")
            .about("Extracts BHD/BDT contents")
            .arg(Arg::with_name("file")
                .help("BHD file path, usually with bhd5 extension")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output directory")
                .short("o").long("output").takes_value(true).required(true))
            .arg(Arg::with_name("namefile")
                .help("Namefile path, mapping hashes to file names")
                .short("n").long("names").takes_value(true).required(false)
                .default_value(default_namefilepath)))
        .subcommand(SubCommand::with_name("bhds")
            .about("Extracts all BHD/BDT content (alphabetically) in a folder")
            .arg(Arg::with_name("folder")
                .help("Path where BHD/BDT archives are stored")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output directory")
                .short("o").long("output").takes_value(true).required(true))
            .arg(Arg::with_name("namefile")
                .help("Namefile path, mapping hashes to file names")
                .short("n").long("names").takes_value(true).required(false)
                .default_value(default_namefilepath)))
        .subcommand(SubCommand::with_name("hash")
            .about("Calculates hash for a string")
            .arg(Arg::with_name("value")
                .help("Any string or path to hash")
                .takes_value(true).required(true)))
        .subcommand(SubCommand::with_name("dcx")
            .about("Extracts and decompress DCX data")
            .arg(Arg::with_name("file")
                .help("DCX path")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output directory")
                .short("o").long("output").takes_value(true).required(false)))
        .subcommand(SubCommand::with_name("bnd")
            .about("Extracts BND contents")
            .arg(Arg::with_name("file")
                .help("BND (or BND/DCX) file path")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output directory")
                .short("o").long("output").takes_value(true).required(true))
            .arg(Arg::with_name("overwrite")
                .help("Overwrite existing files")
                .short("f").long("force").takes_value(false).required(false))
            .arg(Arg::with_name("decompress")
                .help("Decompress file first if BND is in DCX")
                .long("decompress").takes_value(false).required(false)))
        .subcommand(SubCommand::with_name("bhf")
            .about("Extracts BHF/BDT contents")
            .arg(Arg::with_name("file")
                .help("BHF file path")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output directory")
                .short("o").long("output").takes_value(true).required(true))
            .arg(Arg::with_name("overwrite")
                .help("Overwrite existing files")
                .short("f").long("force").takes_value(false).required(false)))
        .subcommand(SubCommand::with_name("paramdef")
            .about("Print PARAMDEF contents")
            .arg(Arg::with_name("file")
                .help("PARAMDEF file path")
                .takes_value(true).required(true)))
        .subcommand(SubCommand::with_name("param")
            .about("Parse PARAM contents")
            .arg(Arg::with_name("file")
                .help("PARAM file path")
                .takes_value(true).required(true))
            .arg(Arg::with_name("paramdef")
                .help("PARAMDEF file path")
                .short("d").long("def").takes_value(true).required(false)))
        .subcommand(SubCommand::with_name("dat")
            .about("Extracts King's Field IV DAT contents")
            .arg(Arg::with_name("file")
                .help("DAT file path")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output directory")
                .short("o").long("output").takes_value(true).required(true)))
        .subcommand(SubCommand::with_name("dat-pack")
            .about("Pack files in a King's Field IV DAT")
            .arg(Arg::with_name("files")
                .help("Directory containing files to pack")
                .takes_value(true).required(true))
            .arg(Arg::with_name("output")
                .help("Output file")
                .takes_value(true).required(true)))
        .get_matches();

    process::exit(match matches.subcommand() {
        ("bhd", Some(s)) => cmd_bhd(s),
        ("bhds", Some(s)) => cmd_bhds(s),
        ("hash", Some(s)) => cmd_hash(s),
        ("dcx", Some(s)) => cmd_dcx(s),
        ("bnd", Some(s)) => cmd_bnd(s),
        ("bhf", Some(s)) => cmd_bhf(s),
        ("paramdef", Some(s)) => cmd_paramdef(s),
        ("param", Some(s)) => cmd_param(s),
        ("dat", Some(s)) => cmd_dat(s),
        ("dat-pack", Some(s)) => cmd_dat_pack(s),
        _ => 0,
    })
}

fn get_default_namefilepath() -> String {
    let program_path: path::PathBuf = env::current_exe().unwrap();
    let program_dir: &path::Path = program_path.parent().unwrap();
    let mut namefile_path: path::PathBuf = path::PathBuf::from(program_dir);
    namefile_path.push("res/namefile.json");
    String::from(namefile_path.to_str().unwrap())
}

fn cmd_bhd(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    let output_path: &str = args.value_of("output").unwrap();

    let namefile_path: &str = args.value_of("namefile").unwrap();
    let names = match name_hashes::load_name_map(namefile_path) {
        Ok(n) => { n }
        Err(e) => { eprintln!("Failed to load namefile: {:?}", e); return 1 }
    };

    match unpackers::bhd::extract_bhd(file_path, &names, output_path) {
        Err(e) => { eprintln!("Failed to extract BHD: {:?}", e); 1 }
        _ => { 0 }
    }
}

fn cmd_bhds(args: &ArgMatches) -> i32 {
    let folder_path: &str = args.value_of("folder").unwrap();
    let output_path: &str = args.value_of("output").unwrap();

    let namefile_path: &str = args.value_of("namefile").unwrap();
    let names = match name_hashes::load_name_map(namefile_path) {
        Ok(n) => { n }
        Err(e) => { eprintln!("Failed to load namefile: {:?}", e); return 1 }
    };

    let entries = match fs::read_dir(folder_path) {
        Ok(o) => { o }
        Err(e) => { eprintln!("Cannot read folder content: {:?}", e); return 1 }
    };
    let mut bhd_paths = vec!();
    for entry in entries {
        if !entry.is_ok() {
            continue
        }
        let path = entry.unwrap().path();
        if let Some(e) = path.extension() {
            if e == "bhd5" {
                bhd_paths.push(path);
            }
        }
    }
    bhd_paths.sort();

    for bhd_path in bhd_paths {
        println!("Extracting {:?}", bhd_path);
        if let Some(path_str) = bhd_path.to_str() {
            if let Err(e) = unpackers::bhd::extract_bhd(path_str, &names, output_path) {
                eprintln!("Failed to extract BHD: {:?}", e);
                return 1
            }
        }
    }
    0
}

fn cmd_hash(args: &ArgMatches) -> i32 {
    let value: &str = args.value_of("value").unwrap();
    println!("{}", name_hashes::hash_as_string(name_hashes::hash(&value)));
    0
}

fn cmd_dcx(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    let output_path: String =
        match unpackers::dcx::get_decompressed_path(file_path, args.value_of("output")) {
            Some(p) => p,
            _ => { return 1 }
        };

    match unpackers::dcx::extract_dcx(file_path, &output_path) {
        Err(e) => { eprintln!("Failed to extract DCX: {:?}", e); 1 }
        _ => 0
    }
}

fn cmd_bnd(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    let output_path: &str = args.value_of("output").unwrap();
    let overwrite: bool = args.is_present("overwrite");
    let decompress: bool = args.is_present("decompress");

    match unpackers::bnd::extract_bnd_file(file_path, output_path, overwrite, decompress) {
        Err(e) => { eprintln!("Failed to extract BND: {:?}", e); 1 }
        _ => 0
    }
}

fn cmd_bhf(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    let output_path: &str = args.value_of("output").unwrap();
    let overwrite: bool = args.is_present("overwrite");
    match unpackers::bhf::extract_bhf_file(file_path, output_path, overwrite) {
        Err(e) => { eprintln!("Failed to extract BHF: {:?}", e); 1 }
        _ => 0
    }
}

fn cmd_paramdef(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    match unpackers::paramdef::load_paramdef_file(file_path) {
        Ok(paramdef) => { unpackers::paramdef::print_paramdef(&paramdef); 0 }
        Err(e) => { eprintln!("Failed to load PARAMDEF: {:?}", e); 1 }
    }
}

fn cmd_param(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    let paramdef_path: Option<&str> = args.value_of("paramdef");

    let paramdef = if paramdef_path.is_some() {
        match unpackers::paramdef::load_paramdef_file(paramdef_path.unwrap()) {
            Ok(paramdef) => Some(paramdef),
            Err(e) => { eprintln!("Failed to load PARAMDEF: {:?}", e); return 1 }
        }
    } else {
        None
    };

    let param = match unpackers::param::load_param_file(file_path, paramdef.as_ref()) {
        Ok(param) => param,
        Err(e) => { eprintln!("Failed to load PARAM: {:?}", e); return 1 }
    };

    match paramdef {
        Some(paramdef) => unpackers::param::print_param_with_def(&param, &paramdef),
        None => unpackers::param::print_param(&param),
    };
    0
}

fn cmd_dat(args: &ArgMatches) -> i32 {
    let file_path: &str = args.value_of("file").unwrap();
    let output_path: &str = args.value_of("output").unwrap();
    match unpackers::dat::extract_dat_file(file_path, output_path) {
        Err(e) => { eprintln!("Failed to extract DAT: {:?}", e); 1 }
        _ => 0
    }
}

fn cmd_dat_pack(args: &ArgMatches) -> i32 {
    let files_path: &str = args.value_of("files").unwrap();
    let output_path: &str = args.value_of("output").unwrap();
    match repackers::dat::pack_dat(files_path, output_path) {
        Err(e) => { eprintln!("Failed to pack DAT: {:?}", e); 1 }
        _ => 0
    }
}
