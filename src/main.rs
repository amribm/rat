use std::io::{self, Result};
use std::{env, fs};

use clap::{Arg, ArgAction, Command};

mod options {
    pub const FILES: &str = "files";
    pub const NUMBER_NON_BLANK: &str = "number non blank lines";
    pub const NUMBER_LINES: &str = "number lines";
    pub const ADD_END_DOLLER: &str = "add `$` at the end of line";
    pub const FAST_PRINT: &str = "fast print";
}

struct Flags {
    number_lines: bool,
    number_non_blank_lines: bool,
    add_dollar_end: bool,
    fast_print: bool,
}

impl Flags {
    fn new() -> Flags {
        Flags {
            number_lines: false,
            number_non_blank_lines: false,
            add_dollar_end: false,
            fast_print: false,
        }
    }
}

fn main() -> Result<()> {
    let mut flags = Flags::new();
    let matches = app().get_matches_from(env::args_os());

    flags.number_non_blank_lines = matches.get_flag(options::NUMBER_NON_BLANK);
    flags.number_lines = matches.get_flag(options::NUMBER_LINES);
    flags.add_dollar_end = matches.get_flag(options::ADD_END_DOLLER);

    let files: Vec<String> = match matches.get_many::<String>(options::FILES) {
        Some(s) => s.map(|x| x.to_string()).collect(),
        None => Vec::new(),
    };

    if flags.number_non_blank_lines {
        flags.number_lines = true;
    }

    for file in files {
        print_file_lines(file, &flags)?;
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
        .arg(
            Arg::new(options::ADD_END_DOLLER)
                .short('e')
                .help("add `$` at the end of the line")
                .action(ArgAction::SetTrue)
                .overrides_with(options::ADD_END_DOLLER),
        )
        .arg(
            Arg::new(options::FAST_PRINT)
                .short('f')
                .help("prints fast")
                .action(ArgAction::SetTrue)
                .overrides_with(options::FAST_PRINT),
        )
        .arg(Arg::new(options::FILES).action(ArgAction::Append))
}

fn print_file_lines(file: String, flags: &Flags) -> io::Result<()> {
    let file_stirng = fs::read_to_string(file)?;
    if flags.fast_print {
        print!("{file_stirng}")
    }

    let mut count = 0;
    for line in file_stirng.lines() {
        let mut line = format!("{line}");
        if flags.number_lines {
            line = number_line(&line, flags.number_non_blank_lines, &mut count);
        }

        if flags.add_dollar_end {
            line.push('$');
        }
        println!("{line}");
    }

    Ok(())
}

fn number_line(line: &str, number_non_blank_lines: bool, count: &mut i32) -> String {
    if line.is_empty() {
        if number_non_blank_lines {
            return format!("{:>6}  {line}", "");
        }
    }
    *count += 1;
    format!("{:>6}  {line}", count)
}
