/// A simple helper for writing tests.
#[allow(unused_macros)]
macro_rules! describe {
    ($function_name:tt => {$($tests:tt)*}) => {
        mod $function_name {
            #[allow(unused_imports)]
            use super::*;

            $($tests)*
        }
    };
}
