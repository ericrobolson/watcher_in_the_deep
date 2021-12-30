use crate::traits::PrettyPrint;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ScriptOptions {
    Directory,
    Ext,
    Name,
    Path,
}

impl ScriptOptions {
    /// Returns the values for the script options.
    pub fn values() -> [Self; 4] {
        [Self::Directory, Self::Ext, Self::Name, Self::Path]
    }
}

impl PrettyPrint for ScriptOptions {
    fn pretty_print(&self) -> String {
        match self {
            ScriptOptions::Directory => "DIR",
            ScriptOptions::Ext => "EXT",
            ScriptOptions::Name => "NAME",
            ScriptOptions::Path => "PATH",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    describe!(pretty_print => {
        #[test]
        fn directory() {
            assert_eq!("DIR", ScriptOptions::Directory.pretty_print());
        }

        #[test]
        fn ext() {
            assert_eq!("EXT", ScriptOptions::Ext.pretty_print());
        }


        #[test]
        fn name() {
            assert_eq!("NAME", ScriptOptions::Name.pretty_print());
        }

        #[test]
        fn path() {
            assert_eq!("PATH", ScriptOptions::Path.pretty_print());
        }
    });

    describe!(values => {
        #[test]
        fn values() {
            assert_eq!([ScriptOptions::Directory, ScriptOptions::Ext, ScriptOptions::Name, ScriptOptions::Path], ScriptOptions::values());
        }
    });
}
