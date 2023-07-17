#![deny(clippy::all)]

use std::{env, process};

fn main() {
    let args = env::args();
    if args.len() < 2 {
        eprintln!("Problem parsing arguments: Not enough arguments");
        process::exit(1)
    }

    let matches = echo_rs::core().get_matches_from(args);

    let no_newline = matches.get_flag(echo_rs::config::NO_NEWLINE);
    let params: Vec<String> = match matches.get_many::<String>(echo_rs::config::PARAM) {
        Some(s) => s.map(|s| s.to_string()).collect(),
        None => Vec::new(),
    };

    if let Err(e) = echo_rs::run(no_newline, &params) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }

}
