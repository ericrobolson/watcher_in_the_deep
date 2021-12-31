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

#[derive(Debug, PartialEq)]
pub struct Witd {
    command: Command,
    files: HashMap<String, File>,
}

impl Witd {
    /// Executes the given instance on the collection of files.
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

    /// Attempts to retrieve the given file.
    fn get_file(&self, file: &File) -> Option<&File> {
        self.files.get(&file.path)
    }

    /// Inserts a file into the hashmap.
    fn insert(&mut self, file: File) {
        self.files.insert(file.path.clone(), file);
    }

    /// Creates a new Watcher in the Deep object
    pub fn new(command: Command) -> Self {
        Self {
            command,
            files: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd() -> Command {
        Command::parse("directory . do echo end").unwrap()
    }

    describe!(execute => {
        #[test]
        fn tests(){
            todo!()
        }
    });

    describe!(get_file => {
        #[test]
        fn returns_none_for_nonexistant_obj(){
            todo!();
        }

        #[test]
        fn returns_some_for_existant_obj(){
            todo!();
        }
    });

    describe!(insert => {
        #[test]
        fn inserts_obj(){
            todo!();
        }
    });

    describe!(new => {
        #[test]
        fn returns_expected(){
            assert_eq!(Witd{ command: cmd(), files: HashMap::new() }, Witd::new(cmd()));
        }
    });
}
