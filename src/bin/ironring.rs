use std::env;
use std::fs;
use std::path;
use std::process;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use rir::{name_hashes, unpackers, utils};

fn main() {
    let default_namefilepath: &str = &get_default_namefilepath();
    let matches = App::new("Rusted Iron Ring")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("bhd")
            .about("Extracts BHD/BDT contents")
            .arg(Arg::with_name("file")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("namefile")
                .short("n")
                .long("names")
                .takes_value(true)
                .required(false)
                .default_value(default_namefilepath)))
        .subcommand(SubCommand::with_name("bhds")
            .about("Extracts all BHD/BDT content (alphabetically) in a folder")
            .arg(Arg::with_name("folder")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("namefile")
                .short("n")
                .long("names")
                .takes_value(true)
                .required(false)
                .default_value(default_namefilepath)))
        .subcommand(SubCommand::with_name("hash")
            .about("Calculates hash for a string")
            .arg(Arg::with_name("value")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("dcx")
            .about("Extracts and decompress DCX data")
            .arg(Arg::with_name("file")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(false)))
        .subcommand(SubCommand::with_name("bnd")
            .about("Extracts BND contents")
            .arg(Arg::with_name("file")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .required(true)))
        .get_matches();

    process::exit(match matches.subcommand() {
        ("bhd", Some(s)) => { cmd_bhd(s) }
        ("bhds", Some(s)) => { cmd_bhds(s) }
        ("hash", Some(s)) => { cmd_hash(s) }
        ("dcx", Some(s)) => { cmd_dcx(s) }
        ("bnd", Some(s)) => { cmd_bnd(s) }
        _ => { 0 }
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

    return match unpackers::bhd::extract_bhd(file_path, &names, output_path) {
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
    let mut output_path_valid = false;
    let mut output_path: String = match args.value_of("output") {
        Some(s) => { output_path_valid = true; s.to_string() }
        _ => { String::with_capacity(file_path.len()) }
    };
    // If no output path is provided, try to strip the file extension.
    if !output_path_valid {
        if let Some(pb) = utils::fs::strip_extension(&path::PathBuf::from(&file_path)) {
            if let Some(s) = pb.to_str() {
                output_path.push_str(s);
                output_path_valid = true;
            }
        }
    }
    if !output_path_valid {
        eprintln!("Could not determine a valid output path.");
        return 1
    }
    // If the output path is a dir, try to strip extension and place the file there.
    if path::Path::new(&output_path).is_dir() {
        output_path_valid = false;
        let mut out_pb = path::PathBuf::from(&output_path);
        if let Some(file_pb) = utils::fs::strip_extension(&path::PathBuf::from(&file_path)) {
            if let Some(file_name) = file_pb.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    out_pb.push(file_name_str);
                    if let Some(s) = out_pb.as_path().to_str() {
                        output_path.clear();
                        output_path.push_str(s);
                        output_path_valid = true;
                    }
                }
            }
        }
    }
    if !output_path_valid {
        eprintln!("Could not determine a valid output path.");
        return 1
    }

    match unpackers::dcx::extract_dcx(file_path, &output_path) {
        Err(e) => { eprintln!("Failed to extract DCX: {:?}", e); return 1 }
        _ => { 0 }
    }
}

fn cmd_bnd(args: &ArgMatches) -> i32 {
    let _file_path: &str = args.value_of("file").unwrap();
    let _output_path: &str = args.value_of("output").unwrap();
    0
}
