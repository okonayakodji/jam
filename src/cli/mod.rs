use std::result::Result;

pub const PROGRAM_DESCRIPTION: &str = "";
pub const HELP_MESSAGE: &str = "Help message";

pub struct Cli {
    pub command: String,
}

pub struct Jam {}

pub fn parse_arguments(args: Vec<String>) -> Result<Cli, &'static str> {
    if args.len() > 1 {
        Ok(Cli {
            command: args[1].clone(),
        })
    } else {
        return Err("need to send a command");
    }
}
