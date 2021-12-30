#[cfg(test)]
#[macro_use]
mod test_helpers;

mod list_all_files;
mod traits;
mod types;
mod witd;

use std::env;
use witd::{Witd, WitdErr};

use crate::{
    traits::PrettyPrint,
    witd::{Command, CommandErr},
};

fn main() -> Result<(), WitdErr> {
    let args: Vec<String> = env::args().collect();

    let input = args
        .iter()
        .skip(1)
        .map(|s| s.clone())
        .collect::<Vec<String>>()
        .join(" ");

    let command = handle_parse_command(&input)?;
    let path = command.root_path().clone();

    let mut witd = Witd::new(command);

    loop {
        let files = list_all_files::execute(&path);
        witd.execute(files)?;
    }
}

fn handle_parse_command(command: &str) -> Result<Command, CommandErr> {
    match Command::parse(command) {
        Ok(command) => Ok(command),
        Err(e) => {
            println!("Error: {}", e.pretty_print());
            println!(
                "{}",
                Command::examples()
                    .iter()
                    .map(|m| format!("Example: {}", m))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            Err(e.into())
        }
    }
}
