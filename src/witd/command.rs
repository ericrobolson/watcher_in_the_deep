use super::{CommandErr, RunMode, ScriptOptions};
use crate::{traits::PrettyPrint, types::File, witd::Keyword};
use std::process;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParseState {
    CheckingCommand,
    CheckingDo,
    CheckingRunMode,
    CheckingPath,
    Fin,
}

/// A command that may be executed.
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    command: String,
    root_path: String,
    run_mode: RunMode,
}
impl Command {
    pub fn parse(s: &str) -> Result<Self, CommandErr> {
        let mut parse_state = ParseState::CheckingRunMode;

        if s.trim().len() == 0 {
            return Err(CommandErr::EmptyInput);
        }

        let mut path = None;
        let mut command: Vec<String> = vec![];

        let split = s.split_ascii_whitespace();
        for token in split {
            match parse_state {
                ParseState::CheckingRunMode => {
                    if token.to_lowercase() != "in" {
                        break;
                    } else {
                        parse_state = ParseState::CheckingPath;
                    }
                }
                ParseState::CheckingPath => {
                    path = Some(token.to_string());
                    parse_state = ParseState::CheckingDo;
                }
                ParseState::CheckingCommand => {
                    if token.to_lowercase() == "end" {
                        parse_state = ParseState::Fin;
                    } else {
                        command.push(token.to_string());
                    }
                }
                ParseState::Fin => {}
                ParseState::CheckingDo => {
                    if token.to_lowercase() != "do" {
                        break;
                    } else {
                        parse_state = ParseState::CheckingCommand;
                    }
                }
            }
        }

        // Final check for state
        let error = match parse_state {
            ParseState::CheckingRunMode => Some(CommandErr::MissingRunMode),
            ParseState::CheckingDo => Some(CommandErr::MissingKeywordDo),
            ParseState::CheckingCommand => Some(CommandErr::MissingKeywordEnd),
            ParseState::CheckingPath => Some(CommandErr::MissingPathSpecification),
            ParseState::Fin => None,
        };

        if let Some(error) = error {
            return Err(error);
        }

        let path = match path {
            Some(path) => path,
            None => return Err(CommandErr::MissingPathSpecification),
        };

        let run_mode = RunMode::File;

        Ok(Self {
            command: command.join(" "),
            root_path: path,
            run_mode,
        })
    }

    /// Returns an example command.
    pub fn examples() -> Vec<String> {
        let ex1 = format!(
            "{} ./src {} echo {options} end",
            RunMode::Directory.pretty_print(),
            Keyword::Do.pretty_print(),
            options = RunMode::Directory
                .allowed_options()
                .iter()
                .map(|m| m.pretty_print())
                .collect::<Vec<String>>()
                .join("|")
        );
        let ex2 = format!(
            "foreach file in ./src do echo {options} end",
            options = RunMode::File
                .allowed_options()
                .iter()
                .map(|m| m.pretty_print())
                .collect::<Vec<String>>()
                .join("|")
        );

        vec![ex1, ex2]
    }

    /// Returns a stringified version of the command to execute.
    fn execution(&self, file: &File) -> String {
        let mut command = self.command.clone();
        for script_option in ScriptOptions::values() {
            let identifier = script_option.pretty_print();
            let value = match script_option {
                ScriptOptions::Directory => &file.directory,
                ScriptOptions::Ext => &file.extension,
                ScriptOptions::Name => &file.name,
                ScriptOptions::Path => &file.path,
            };

            command = command.replace(&identifier, value);
        }

        command
    }

    /// Executes the given command on the given file.
    pub fn execute(&self, file: &File) {
        // rather hacky, but this will build up a command to execute by splitting off the tokens and the like.
        let cmd = self.execution(file);

        let mut tokens = vec![];
        for token in cmd.split_ascii_whitespace() {
            tokens.push(token.to_string());
        }

        let cmd = tokens[0].clone();
        let args: Vec<String> = tokens.iter().skip(1).map(|s| s.clone()).collect();

        let output = process::Command::new(cmd)
            .args(args)
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8(output.stdout).unwrap();
        println!("{}", output);
    }

    /// Returns the root path for the command.
    pub fn root_path(&self) -> &String {
        &self.root_path
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn file() -> File {
        File {
            created_at: Duration::from_millis(333),
            directory: "./testy".into(),
            extension: "obj".into(),
            modified_at: Duration::from_millis(444),
            name: "testy-mctest.obj".into(),
            path: "./testy/testy-mctest.obj".into(),
        }
    }

    fn cmd() -> Command {
        Command {
            command: "echo NAME".into(),
            root_path: "".into(),
            run_mode: RunMode::File,
        }
    }

    describe!(examples => {
        #[test]
        fn examples() {
            assert_eq!(vec!["directory ./src do echo DIR end".to_string(), "foreach file in ./src do echo DIR|EXT|NAME|PATH end".to_string()], Command::examples());
        }
    });

    describe!(execution => {
       #[test]
        fn dir_replaces_filename(){
            let mut command = cmd();
            command.command = "echo DIR".into();

            let expected = format!("echo {}", file().directory);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn name_replaces_filename(){
            let mut command = cmd();
            command.command = "echo NAME".into();

            let expected = format!("echo {}", file().name);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn path_replaces_path(){
            let command = Command{ command: "echo PATH".into(), root_path: "".into(), run_mode: RunMode::File };

            let expected = format!("echo {}", file().path);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn ext_replaces_ext(){
            let command = Command{ command: "echo EXT".into(), root_path: "".into(), run_mode: RunMode::File };

            let expected = format!("echo {}", file().extension);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn complex(){
            let command = Command{ command: "echo testy_NAME_path_PATH_ext_EXT".into(), root_path: "".into(), run_mode: RunMode::File };

            let expected = format!("echo testy_{name}_path_{path}_ext_{ext}", name = file().name, path = file().path, ext = file().extension);
            assert_eq!(expected, command.execution(&file()));
        }
    });

    describe!(parse => {
       #[test]
        fn empty_input_returns_error(){
            let input = "  ";
            let expected = Err(CommandErr::EmptyInput);

            assert_eq!(expected, Command::parse(input));
        }

        #[test]
        fn missing_keyword_in(){
            let input = " . execute \"echo {filename}\"";
            let expected = Err(CommandErr::MissingRunMode);

            assert_eq!(expected, Command::parse(input));
        }

        #[test]
        fn missing_path_specification(){
            let input = "in ";
            let expected = Err(CommandErr::MissingPathSpecification);

            assert_eq!(expected, Command::parse(input));
        }

        #[test]
        fn missing_keyword_do(){
            let input = "in . ";
            let expected = Err(CommandErr::MissingKeywordDo);

            assert_eq!(expected, Command::parse(input));
        }

        #[test]
        fn missing_keyword_end(){
            let input = "in . do some random stuff";
            let expected = Err(CommandErr::MissingKeywordEnd);

            assert_eq!(expected, Command::parse(input));
        }

        #[test]
        fn happy_path_no_where_clauses(){
            let input = "in . do some random stuff end";
            let expected = Ok(Command{ root_path: ".".into(), command: "some random stuff".into(), run_mode: RunMode::File });

            assert_eq!(expected, Command::parse(input));
        }
    });
}
