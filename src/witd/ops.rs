use crate::types::File;

// Example usage:
// in "." execute "echo {filename}"
// in "." execute "cp {path} ./output_{filename}"

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseError {
    MissingPathSpecification,
}

pub struct Command {
    command: String,
}
impl Command {
    pub fn parse(s: &str) -> Result<Self, ()> {
        // TODO: lowercase stuff?
        todo!()
    }

    /// Returns a stringified version of the command to execute.
    pub fn execution(&self, file: &File) -> String {
        self.command
            .replace("{path}", &file.path)
            .replace("{name}", &file.name)
            .replace("{ext}", &file.extension)
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
            let command = Command{ command: "echo {name}".into() };

            let expected = format!("echo {}", file().name);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn path_replaces_path(){
            let command = Command{ command: "echo {path}".into() };

            let expected = format!("echo {}", file().path);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn ext_replaces_ext(){
            let command = Command{ command: "echo {ext}".into() };

            let expected = format!("echo {}", file().extension);
            assert_eq!(expected, command.execution(&file()));
        }

        #[test]
        fn complex(){
            let command = Command{ command: "echo testy_{name}_path_{path}_ext_{ext}".into() };

            let expected = format!("echo testy_{name}_path_{path}_ext_{ext}", name = file().name, path = file().path, ext = file().extension);
            assert_eq!(expected, command.execution(&file()));
        }
    });
}
