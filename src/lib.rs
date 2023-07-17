use std::{error::Error, io::{self, Write}};

use clap::{Command, Arg, ArgAction};

pub mod config {
    pub const NO_NEWLINE: &str = "no_newline";
    pub const PARAM: &str = "string";
}

pub fn core() -> Command {
    Command::new("echo")
        .version("1.1")
        .about("about")
        .arg(
            Arg::new(config::NO_NEWLINE)
                .short('n')
                .long("no-line")
                .action(ArgAction::SetTrue)
            )
        .arg(Arg::new(config::PARAM).action(ArgAction::Append))
}

pub fn run(no_newline: bool, params: &[String]) -> Result<(), Box<dyn Error>> {
    let mut writer = io::stdout().lock();
    
    for output in params.iter() {
        write!(writer, "{}", output)?;
    }

    if !no_newline {
        writeln!(writer)?;
    }

    Ok(())
}