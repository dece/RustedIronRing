use std::fs::File;
use std::io::{Error, Read};

extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

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
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("bhd", Some(s)) => { cmd_bhd(s).unwrap(); }
        _ => {}
    }
}

fn cmd_bhd(args: &ArgMatches) -> Result::<(), Error> {
    let filepath: &str = args.value_of("file").unwrap();
    let outputpath: &str = args.value_of("output").unwrap();
    println!("File: {:?}", filepath);
    println!("Output: {:?}", outputpath);

    let mut bhd_file: File = File::open(filepath)?;
    let mut bhd_data = vec![0u8; bhd_file.metadata()?.len() as usize];
    bhd_file.read_to_end(&mut bhd_data)?;

    bhd::parse(&bhd_data);
    Ok(())
}
