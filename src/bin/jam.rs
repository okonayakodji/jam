use std::{env, process::exit};

use jam::cli::{parse_arguments, Cli, HELP_MESSAGE};

fn main() {
    let cli: Cli = match parse_arguments(env::args().collect()) {
        Ok(c) => c,
        Err(_e) => {
            println!("{}", HELP_MESSAGE);
            exit(1);
        }
    };
    println!("{}", cli.command);
}
