mod command;
use std::collections::HashMap;

pub use command::*;

use crate::types::File;

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

/// An operation to perform.
pub enum Op {}

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
                self.command.execute(file);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    describe!(add_path => {

    });

    describe!(paths => {

    });
}
