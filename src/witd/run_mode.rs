use crate::traits::PrettyPrint;

use super::ScriptOptions;

/// The list of run modes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RunMode {
    Directory,
    File,
}
impl RunMode {
    /// Returns the allowed script options for the run mode.
    pub fn allowed_options(&self) -> Vec<ScriptOptions> {
        match self {
            RunMode::Directory => vec![ScriptOptions::Directory],
            RunMode::File => vec![
                ScriptOptions::Directory,
                ScriptOptions::Ext,
                ScriptOptions::Name,
                ScriptOptions::Path,
            ],
        }
    }

    /// The allowed string values for the run mode.
    pub fn values() -> [Self; 2] {
        [Self::Directory, Self::File]
    }
}

impl PrettyPrint for RunMode {
    fn pretty_print(&self) -> String {
        match self {
            RunMode::Directory => "directory".into(),
            RunMode::File => "file".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    describe!(allowed_options => {
        #[test]
        fn directory() {
            let expected = vec![ScriptOptions::Directory];
            assert_eq!(expected, RunMode::Directory.allowed_options());
        }

        #[test]
        fn file() {
            let expected = vec![ScriptOptions::Directory, ScriptOptions::Ext, ScriptOptions::Name, ScriptOptions::Path];
            assert_eq!(expected, RunMode::File.allowed_options());
        }
    });

    describe!(pretty_print => {
        #[test]
        fn per_directory(){
            assert_eq!("directory", RunMode::Directory.pretty_print());
        }

        #[test]
        fn per_file(){
            assert_eq!("file", RunMode::File.pretty_print());
        }
    });

    describe!(values => {
        #[test]
        fn returns_expected(){
            assert_eq!([RunMode::Directory, RunMode::File], RunMode::values());
        }
    });
}
