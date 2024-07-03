use std::io::{self, Result};
use std::{env, fs};

use clap::{Arg, ArgAction, Command};

mod options {
    pub const FILES: &str = "files";
}

fn main() -> Result<()> {
    let matches = app().get_matches_from(env::args_os());

    let files: Vec<String> = match matches.get_many::<String>(options::FILES) {
        Some(s) => s.map(|x| x.to_string()).collect(),
        None => Vec::new(),
    };

    for file in files {
        print_file_lines(file)?;
    }

    Ok(())
}

fn app() -> Command {
    Command::new("rat")
        .bin_name("/target/debug/rat")
        .about("simple `cat` like command line uitility")
        .arg(Arg::new(options::FILES).action(ArgAction::Append))
}

fn print_file_lines(file: String) -> io::Result<()> {
    let file_stirng = fs::read_to_string(file)?;

    for line in file_stirng.lines() {
        println!("{line}");
    }

    Ok(())
}
