use std::env::current_exe;
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};

//extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

//extern crate nom;
use nom::Err::{Error as NomError, Failure as NomFailure};

mod parsers {
    pub mod bhd;
}
use parsers::*;

fn main() {
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
                .required(false)))
        .get_matches();

    match matches.subcommand() {
        ("bhd", Some(s)) => { cmd_bhd(s).unwrap(); }
        _ => {}
    }
}

fn cmd_bhd(args: &ArgMatches) -> Result::<(), Error> {
    let filepath: &str = args.value_of("file").unwrap();
    let outputpath: &str = args.value_of("output").unwrap();
    let namefilepath: &str = args.value_of("namefile").unwrap_or(&get_default_namefilepath());
    let mut bhd_file: File = File::open(filepath)?;
    let file_len = bhd_file.metadata()?.len() as usize;
    let mut bhd_data = vec![0u8; file_len];
    bhd_file.read_exact(&mut bhd_data)?;

    let bhd = match bhd::parse(&bhd_data) {
        Ok((_, bhd)) => { println!("BHD: {:?}", bhd); bhd }
        Err(NomError(e)) | Err(NomFailure(e)) => {
            let (_, kind) = e;
            let reason = format!("{:?} {:?}", kind, kind.description());
            eprintln!("BHD parsing failed: {}", reason); return Ok(())
        }
        e => {
            eprintln!("Unknown error: {:?}", e); return Ok(())
        }
    };

    Ok(())
}

fn get_default_namefilepath() -> String {
    let programpath: PathBuf = current_exe().unwrap();
    let programdir: &Path = programpath.parent().unwrap();
    let mut namefilepath: PathBuf = PathBuf::from(programdir);
    namefilepath.push("res/namefile.json");
    String::from(namefilepath.to_str().unwrap())
}
