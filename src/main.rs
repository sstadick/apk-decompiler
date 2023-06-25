mod decompiler;

use crate::decompiler::Decompiler;
use clap::{App, Arg};
use console::style;
use std::path::PathBuf;

fn main() {
    let matches = App::new("apk decompiler")
        .version("0.2.1")
        .about("A super simple utility to decompile your APKs.")
        .author("Roberto Huertas <roberto.huertas@outlook.com>")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .default_value(".")
                .takes_value(true)
                .index(1)
                .help("The path to your apk."),
        )
        .arg(
            Arg::with_name("libs_dir")
                .long("libs-dir")
                .short("l")
                .takes_value(true)
                .help("The path to the directory containing the libraries (ex: ./libs)."),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let apk_path = PathBuf::from(file_path);
    let dec = Decompiler::new(
        apk_path,
        matches.value_of("libs_dir").unwrap().to_owned().into(),
    );

    if let Err(e) = dec.start() {
        eprintln!("{}", style(format!("  Error: {}", e.to_string())).red());
    }
}
