use crate::traits::PrettyPrint;

/// The list of keywords.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
    Do,
    End,
    Mode,
}
impl PrettyPrint for Keyword {
    fn pretty_print(&self) -> String {
        match self {
            Keyword::Do => "do".into(),
            Keyword::End => "end".into(),
            Keyword::Mode => "mode".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    describe!(pretty_print => {
        #[test]
        fn case_do(){
            assert_eq!("do", Keyword::Do.pretty_print());
        }

        #[test]
        fn case_end(){
            assert_eq!("end", Keyword::End.pretty_print());
        }

        #[test]
        fn case_mode(){
            assert_eq!("mode", Keyword::Mode.pretty_print());
        }
    });
}
