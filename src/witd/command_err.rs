use crate::traits::PrettyPrint;

use super::{Keyword, RunMode};

/// An error that may occur for a command.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandErr {
    EmptyInput,
    MissingKeywordDo,
    MissingKeywordEnd,
    MissingRunMode,
    MissingPathSpecification,
}

impl PrettyPrint for CommandErr {
    fn pretty_print(&self) -> String {
        match self {
            CommandErr::EmptyInput => "Empty input provided!".into(),
            CommandErr::MissingKeywordDo => format!("Missing '{}'!", Keyword::Do.pretty_print()),
            CommandErr::MissingKeywordEnd => format!("Missing '{}'!", Keyword::End.pretty_print()),
            CommandErr::MissingRunMode => format!(
                "Missing '{}'; options are [{}].",
                Keyword::Mode.pretty_print(),
                RunMode::values()
                    .iter()
                    .map(|m| m.pretty_print())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            CommandErr::MissingPathSpecification => "Missing path specification!".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::witd::Keyword;

    describe!(pretty_print => {
        #[test]
        fn empty_input(){
            assert_eq!("Empty input provided!", CommandErr::EmptyInput.pretty_print());
        }

        #[test]
        fn missing_keyword_do(){
            assert_eq!(format!("Missing '{}'!", Keyword::Do.pretty_print()), CommandErr::MissingKeywordDo.pretty_print());
        }

        #[test]
        fn missing_keyword_end(){
            assert_eq!(format!("Missing '{}'!", Keyword::End.pretty_print()), CommandErr::MissingKeywordEnd.pretty_print());
        }

        #[test]
        fn missing_run_mode(){
            assert_eq!(format!("Missing 'mode'; options are [directory, file]."), CommandErr::MissingRunMode.pretty_print());
        }

        #[test]
        fn missing_path_specification(){
            assert_eq!("Missing path specification!", CommandErr::MissingPathSpecification.pretty_print());
        }
    });
}
