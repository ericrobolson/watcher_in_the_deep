use super::{CommandErr, RunMode, ScriptOptions};
use crate::{traits::PrettyPrint, types::File, witd::Keyword};
use std::process;

/// A command that may be executed.
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    command: String,
    root_path: String,
    run_mode: RunMode,
}
impl Command {
    /// Parses the given command.
    pub fn parse(s: &str) -> Result<Self, CommandErr> {
        // Check if empty input
        if s.trim().len() == 0 {
            return Err(CommandErr::EmptyInput);
        }

        // Calculate run mode
        let (run_mode, s) = {
            let file_mode = format!("foreach {} in", RunMode::File.pretty_print());
            let directory_mode = RunMode::Directory.pretty_print();

            let is_file_mode = s.starts_with(&file_mode);
            let is_directory_mode = s.starts_with(&directory_mode);

            if !is_file_mode && !is_directory_mode {
                return Err(CommandErr::MissingRunMode);
            }

            let mode = if is_directory_mode {
                RunMode::Directory
            } else {
                RunMode::File
            };

            let s = s
                .to_string()
                .replace(&file_mode, "")
                .replace(&directory_mode, "")
                .trim()
                .to_string();

            (mode, s)
        };

        if s.trim().is_empty() {
            return Err(CommandErr::MissingPathSpecification);
        }

        // Make path
        let (root_path, s) = {
            let mut path = String::default();

            for c in s.chars() {
                if c.is_whitespace() {
                    break;
                } else {
                    path.push(c);
                }
            }

            let s = s.replace(&path, "").trim().to_string();

            (path, s)
        };

        // Make do
        let s = {
            let keyword = Keyword::Do.pretty_print();
            if !s.starts_with(&keyword) {
                return Err(CommandErr::MissingKeywordDo);
            }

            s.replace(&keyword, "").trim().to_string()
        };

        // Parse command
        let (command, s) = {
            let mut command = String::default();
            let end = Keyword::End.pretty_print();
            let mut found_end = s.starts_with(&end);
            let mut s = s;

            if !found_end {
                while s.len() > 0 {
                    let c = s.remove(0);

                    if s.starts_with(&end) && c.is_whitespace() {
                        found_end = true;
                        s = s.replacen(&end, "", 1);
                        break;
                    } else {
                        command.push(c);
                    }
                }
            }

            if !found_end {
                return Err(CommandErr::MissingKeywordEnd);
            }

            (command, s)
        };

        // TODO: where clauses

        // TODO: ensure only whitelisted values are present

        Ok(Self {
            command,
            root_path,
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
    fn execution(&self, file: Option<&File>) -> String {
        let mut command = self.command.clone();

        if let Some(file) = file {
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
        }

        command = command.replace(&ScriptOptions::Directory.pretty_print(), &self.root_path);

        command
    }

    /// Executes the given command on the given file.
    pub fn execute(&self, file: Option<&File>) {
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

    /// Returns the run mode for the command.
    pub fn run_mode(&self) -> RunMode {
        self.run_mode
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
            root_path: "testy/test/src".into(),
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
            assert_eq!(expected, command.execution(Some(&file())));
        }

          #[test]
        fn dir_replaces_filename_if_no_file(){
            let mut command = cmd();
            command.command = "echo DIR".into();

            let expected = format!("echo {}", cmd().root_path());
            assert_eq!(expected, command.execution(None));
        }

        #[test]
        fn name_replaces_filename(){
            let mut command = cmd();
            command.command = "echo NAME".into();

            let expected = format!("echo {}", file().name);
            assert_eq!(expected, command.execution(Some(&file())));
        }

        #[test]
        fn path_replaces_path(){
            let command = Command{ command: "echo PATH".into(), root_path: "".into(), run_mode: RunMode::File };

            let expected = format!("echo {}", file().path);
            assert_eq!(expected, command.execution(Some(&file())));
        }

        #[test]
        fn ext_replaces_ext(){
            let command = Command{ command: "echo EXT".into(), root_path: "".into(), run_mode: RunMode::File };

            let expected = format!("echo {}", file().extension);
            assert_eq!(expected, command.execution(Some(&file())));
        }

        #[test]
        fn complex(){
            let command = Command{ command: "echo testy_NAME_path_PATH_ext_EXT".into(), root_path: "".into(), run_mode: RunMode::File };

            let expected = format!("echo testy_{name}_path_{path}_ext_{ext}", name = file().name, path = file().path, ext = file().extension);
            assert_eq!(expected, command.execution(Some(&file())));
        }
    });

    describe!(parse => {
        fn parse(input: &str) -> Result<Command, CommandErr>{
            Command::parse(input)
        }

        #[test]
        fn empty_input_returns_err(){
            let input = " ";

            let expected = Err(CommandErr::EmptyInput);
            let actual = parse(input);

            assert_eq!(expected, actual);
        }

        #[test]
        fn missing_run_mode_returns_err(){
            let input = " garbage day";

            let expected = Err(CommandErr::MissingRunMode);
            let actual = parse(input);

            assert_eq!(expected, actual);
        }

        const MODES: [(RunMode, &'static str); 2] = [
            (RunMode::Directory,"directory"),
            (RunMode::File, "foreach file in")
        ];

        #[test]
        fn missing_path_returns_err(){
            for (_, mode) in MODES{
                let input = format!("{}", mode);

                let expected = Err(CommandErr::MissingPathSpecification);
                let actual = parse(&input);

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn missing_keyword_do(){
            for (_, mode) in MODES{
                let input = format!("{} ./src/path end", mode);

                let expected = Err(CommandErr::MissingKeywordDo);
                let actual = parse(&input);

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn missing_keyword_end(){
            for (_, mode) in MODES{
                let input = format!("{} ./src/path do some gobblygook", mode);

                let expected = Err(CommandErr::MissingKeywordEnd);
                let actual = parse(&input);

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn happy_path_empty_command(){
            for (run_mode, mode) in MODES{
                let input = format!("{} ./src/path do end", mode);

                let expected = Ok(Command{ command:
                    "".into(), root_path: "./src/path".into(), run_mode });
                let actual = parse(&input);

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn happy_path_with_command(){
            for (run_mode, mode) in MODES{
                let input = format!("{} ./src/path do echo \"HI\" end", mode);

                let expected = Ok(Command{ command:
                    "echo \"HI\"".into(), root_path: "./src/path".into(), run_mode });
                let actual = parse(&input);

                assert_eq!(expected, actual);
            }
        }

        #[test]
        fn validate_command_values(){
          todo!("Ensure that interpolated values are whitelisted")
        }


    });
}
