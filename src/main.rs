#[cfg(test)]
#[macro_use]
mod test_helpers;

mod list_all_files;
mod types;
mod witd;

use std::env;
use witd::{Witd, WitdErr};

use crate::witd::{Command, CommandErr};

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
            let (error, example) = match e {
                CommandErr::EmptyInput => ("Empty input was provided.", "in . do echo {name} end"),
                CommandErr::MissingKeywordDo => {
                    ("Missing keyword 'do'.", "in . do echo {name} end")
                }

                CommandErr::MissingKeywordEnd => {
                    ("Missing keyword 'end'.", "in . do echo {name} end")
                }
                CommandErr::MissingKeywordIn => {
                    ("Missing keyword 'in'.", "in . do echo {name} end")
                }
                CommandErr::MissingPathSpecification => {
                    ("Missing path specification.", "in . do echo {name} end")
                }
            };

            println!("");
            println!("");
            println!("Error: {}", error);
            println!("Example input: {}", example);
            println!("");
            println!("");

            todo!("Instead of hardcoding the example input, instead compose it? Need to make it agnostic to updates.");

            Err(e.into())
        }
    }
}
