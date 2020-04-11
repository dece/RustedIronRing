extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("Rusted Iron Ring")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("bhd")
            .about("Extracts BHD/BDT contents")
            .arg(Arg::with_name("file")
                .takes_value(true)
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("bhd", Some(s)) => { cmd_bhd(s); }
        _ => {}
    }
}

fn cmd_bhd(args: &ArgMatches) {
    let filepath: &str = args.value_of("file").unwrap();
    println!("File: {:?}", filepath);
}
