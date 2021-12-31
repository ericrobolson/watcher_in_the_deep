#[cfg(test)]
#[macro_use]
mod test_helpers;

mod list_all_files;
mod traits;
mod types;
mod witd;

use crate::{
    traits::PrettyPrint,
    witd::{Command, CommandErr, Witd, WitdErr},
};
use std::env;

/// A simple structure containing information related to each Witd instance.
struct Context {
    path: String,
    witd: Witd,
}

/// Builds out a collection of Contexts from a collection of inputs.
fn build_contexts(inputs: Vec<String>) -> Result<Vec<Context>, WitdErr> {
    let mut contexts = vec![];

    for input in inputs {
        let command = handle_parse_command(&input)?;
        contexts.push(Context {
            path: command.root_path().clone(),
            witd: Witd::new(command),
        });
    }

    Ok(contexts)
}

/// Source the input from the env args
fn get_cli_input() -> String {
    let args: Vec<String> = env::args().collect();

    args.iter()
        .skip(1)
        .map(|s| s.clone())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Attempts to parse the command from a string.
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

/// Given some input, split it into a collection of different inputs.
fn split_commands(input: String) -> Vec<String> {
    if input.contains(";;") {
        input
            .split(";;")
            .map(|m| m.trim().to_string())
            .collect::<Vec<String>>()
    } else {
        vec![input]
    }
}

fn main() -> Result<(), WitdErr> {
    // Source all data
    let input = get_cli_input();
    let inputs = split_commands(input);
    let mut contexts = build_contexts(inputs)?;

    // Now constantly loop, executing the files and commands
    loop {
        for context in contexts.iter_mut() {
            let files = list_all_files::execute(&context.path);
            context.witd.execute(files)?;
        }
    }
}
