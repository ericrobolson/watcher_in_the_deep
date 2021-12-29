mod ops;

/// An error that may be returned by WITD.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WError {}

/// An operation to perform.
pub enum Op {}

pub struct Witd {
    paths: Vec<String>,
    should_exit: bool,
}

impl Witd {
    /// Adds a path to WITD
    pub fn add_path(&mut self, path: &str) -> Result<(), WError> {
        self.paths.push(path.to_string());
        self.paths.dedup();

        Ok(())
    }

    /// Returns the list of registered paths
    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    // TODO: tests
    pub fn new() -> Self {
        Self {
            paths: vec![],
            should_exit: false,
        }
    }

    pub fn execute(&mut self, op: Op) {}

    // TODO: tests
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
    // TODO: tests
    pub fn quit(&mut self) {
        self.should_exit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    describe!(add_path => {
        #[test]
        fn adds_path(){
            let mut witd = Witd::new();

            let path: String = "testy.msct".into();
            let res = witd.add_path(&path);

            assert_eq!(Ok(()), res);
            assert_eq!(vec![path], witd.paths);
        }

        #[test]
        fn dedups_duplicate_paths(){
            let mut witd = Witd::new();

            let path: String = "testy.msct".into();

            let res = witd.add_path(&path);
            assert_eq!(Ok(()), res);

            let res = witd.add_path(&path);
            assert_eq!(Ok(()), res);

            let res = witd.add_path(&path);
            assert_eq!(Ok(()), res);

            assert_eq!(vec![path], witd.paths);
        }
    });

    describe!(paths => {
        #[test]
        fn returns_empty(){
            let expected: Vec<String> = vec![];
            let witd = Witd::new();

            assert_eq!(&expected, witd.paths());
        }

          #[test]
        fn returns_expected(){
            let expected: Vec<String> = vec!["testy".into(), "foo bar".into()];
            let mut witd = Witd::new();
            witd.paths = expected.clone();

            assert_eq!(&expected, witd.paths());
        }
    });
}
