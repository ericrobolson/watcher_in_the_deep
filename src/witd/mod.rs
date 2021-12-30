mod command;
mod command_err;
mod keywords;
mod run_mode;
mod script_options;

use crate::types::File;
pub use command::*;
pub use command_err::*;
pub use keywords::*;
pub use run_mode::*;
pub use script_options::*;
use std::collections::HashMap;

/// An error that may be returned by WITD.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WitdErr {
    CommandErr(CommandErr),
}
impl From<CommandErr> for WitdErr {
    fn from(e: CommandErr) -> Self {
        Self::CommandErr(e)
    }
}

pub struct Witd {
    command: Command,
    files: HashMap<String, File>,
}

impl Witd {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            files: HashMap::new(),
        }
    }

    // TODO: tests
    fn insert(&mut self, file: File) {
        self.files.insert(file.path.clone(), file);
    }

    // TODO: tests
    fn get_file(&self, file: &File) -> Option<&File> {
        self.files.get(&file.path)
    }

    pub fn execute(&mut self, files: Vec<File>) -> Result<(), WitdErr> {
        let mut execute_directory = false;
        for file in files.iter() {
            let should_execute = match self.get_file(file) {
                Some(existing) => {
                    if existing.is_older(file) {
                        self.insert(file.clone());
                        true
                    } else {
                        false
                    }
                }
                None => {
                    self.insert(file.clone());
                    true
                }
            };

            if should_execute {
                if self.command.run_mode() == RunMode::File {
                    self.command.execute(Some(file));
                } else {
                    execute_directory = true;
                }
            }
        }

        if execute_directory {
            self.command.execute(None);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        todo!("WITD tests!");
    }

    describe!(add_path => {

    });

    describe!(paths => {

    });
}
