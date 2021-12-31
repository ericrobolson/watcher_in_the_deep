/// A simple trait for pretty printing values.
pub trait PrettyPrint {
    /// Pretty prints the given instance.
    fn pretty_print(&self) -> String;
}
