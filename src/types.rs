use std::time::Duration;

/// A simple struct that represents a file.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct File {
    pub created_at: Duration,
    pub directory: String,
    pub extension: String,
    pub modified_at: Duration,
    pub name: String,
    pub path: String,
}

impl File {
    /// Returns whether the file is older than the other file.
    pub fn is_older(&self, other: &Self) -> bool {
        self.created_at < other.created_at || self.modified_at < other.modified_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file() -> File {
        File {
            created_at: Duration::from_secs_f32(3.9),
            directory: "Test//test".into(),
            extension: "ext".into(),
            modified_at: Duration::from_secs_f32(3.9),
            name: "test".into(),
            path: "test".into(),
        }
    }

    describe!(is_older => {
        #[test]
        fn all_dates_equal_returns_false(){
            let a = file();
            let b = a.clone();

            assert_eq!(false, a.is_older(&b));
        }

        #[test]
        fn created_at_younger_than_other_returns_false(){
            let a = file();
            let mut b = a.clone();
            b.created_at -= Duration::from_nanos(444);
            assert_eq!(false, a.is_older(&b));
        }

        #[test]
        fn created_at_older_than_other_returns_true(){
            let a = file();
            let mut b = a.clone();
            b.created_at += Duration::from_nanos(444);
            assert_eq!(true, a.is_older(&b));
        }

          #[test]
        fn modified_at_younger_than_other_returns_false(){
            let a = file();
            let mut b = a.clone();
            b.modified_at -= Duration::from_nanos(444);
            assert_eq!(false, a.is_older(&b));
        }

        #[test]
        fn modified_at_older_than_other_returns_true(){
            let a = file();
            let mut b = a.clone();
            b.modified_at += Duration::from_nanos(444);
            assert_eq!(true, a.is_older(&b));
        }
    });
}
