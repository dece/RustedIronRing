extern crate clap;

use std::fs::File;
use std::io::{BufReader, Error, Read};

//use std::path::Path;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

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

    let bhd_data: File = File::open(filepath)?;
    let mut bhd_reader = BufReader::new(bhd_data);
    let mut magic: [u8; 4] = [0; 4];
    bhd_reader.read(&mut magic)?;

    println!("First byte: 0x{:X}", magic[0]);
    Ok(())
}
