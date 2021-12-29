use crate::types::File;
use std::process;

/// An error that may occur for a command.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandErr {
    EmptyInput,
    MissingKeywordDo,
    MissingKeywordEnd,
    MissingKeywordIn,
    MissingPathSpecification,
}

#[derive(Clone, Debug, PartialEq)]
enum ParseState {
    CheckingCommand,
    CheckingDo,
    CheckingIn,
    CheckingPath,
    Fin,
}

/// A command that may be executed.
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    root_path: String,
    command: String,
}
impl Command {
    pub fn parse(s: &str) -> Result<Self, CommandErr> {
        let mut parse_state = ParseState::CheckingIn;

        if s.trim().len() == 0 {
            return Err(CommandErr::EmptyInput);
        }

        let mut path = None;
        let mut command: Vec<String> = vec![];

        let split = s.split_ascii_whitespace();
        for token in split {
            match parse_state {
                ParseState::CheckingIn => {
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
            ParseState::CheckingIn => Some(CommandErr::MissingKeywordIn),
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

        Ok(Self {
            root_path: path,
            command: command.join(" "),
        })
    }

    /// Returns a stringified version of the command to execute.
    fn execution(&self, file: &File) -> String {
        self.command
            .replace("PATH", &file.path)
            .replace("NAME", &file.name)
            .replace("EXT", &file.extension)
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
            extension: "obj".into(),
            modified_at: Duration::from_millis(444),
            name: "testy-mctest.obj".into(),
            path: "./testy/testy-mctest.obj".into(),
        }
    }

    describe!(execution => {
        #[test]
        fn name_replaces_filename(){
            let command = Command{ command: "echo NAME".into(), root_path: "".into() };

            let expected = format!("echo {}", file().name);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn path_replaces_path(){
            let command = Command{ command: "echo PATH".into(), root_path: "".into() };

            let expected = format!("echo {}", file().path);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn ext_replaces_ext(){
            let command = Command{ command: "echo EXT".into(), root_path: "".into() };

            let expected = format!("echo {}", file().extension);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn complex(){
            let command = Command{ command: "echo testy_NAME_path_PATH_ext_EXT".into(), root_path: "".into() };

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
            let expected = Err(CommandErr::MissingKeywordIn);

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
            let expected = Ok(Command{ root_path: ".".into(), command: "some random stuff".into() });

            assert_eq!(expected, Command::parse(input));
        }
    });
}
