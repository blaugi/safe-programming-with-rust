pub trait Describable {
    fn describe(&self) -> String;
    fn formatted_string(&self) -> String {
        format!("++++++++++\n{}\n++++++++++", self.describe())
    }
}