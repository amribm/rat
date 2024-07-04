use std::io::{self, Result};
use std::{env, fmt, fs};

use clap::{Arg, ArgAction, Command};

mod options {
    pub const FILES: &str = "files";
    pub const NUMBER_NON_BLANK: &str = "number non blank lines";
    pub const NUMBER_LINES: &str = "number lines";
}

fn main() -> Result<()> {
    let matches = app().get_matches_from(env::args_os());

    let number_non_blank_lines = matches.get_flag(options::NUMBER_NON_BLANK);
    let mut number_lines = matches.get_flag(options::NUMBER_LINES);

    let files: Vec<String> = match matches.get_many::<String>(options::FILES) {
        Some(s) => s.map(|x| x.to_string()).collect(),
        None => Vec::new(),
    };

    if number_non_blank_lines {
        number_lines = true;
    }

    for file in files {
        print_file_lines(file, number_lines, number_non_blank_lines)?;
    }

    Ok(())
}

fn app() -> Command {
    Command::new("rat")
        .bin_name("/target/debug/rat")
        .about("simple `cat` like command line uitility")
        .arg(
            Arg::new(options::NUMBER_NON_BLANK)
                .short('b')
                .help("number non blank lines")
                .action(ArgAction::SetTrue)
                .overrides_with(options::NUMBER_NON_BLANK),
        )
        .arg(
            Arg::new(options::NUMBER_LINES)
                .short('n')
                .help("number non blank lines")
                .action(ArgAction::SetTrue)
                .overrides_with(options::NUMBER_NON_BLANK),
        )
        .arg(Arg::new(options::FILES).action(ArgAction::Append))
}

fn print_file_lines(file: String, number_lines: bool, non_blank_lines: bool) -> io::Result<()> {
    let file_stirng = fs::read_to_string(file)?;

    let mut count = 0;
    for line in file_stirng.lines() {
        let mut line = format!("{line}");
        if number_lines {
            if line.is_empty() {
                if non_blank_lines {
                    println!("");
                    continue;
                }
            }
            count += 1;
            line = format!("{:>6}  {line}", count);
        }
        println!("{line}");
    }

    Ok(())
}
