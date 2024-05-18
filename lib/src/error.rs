#[derive(Debug)]
pub struct Error(pub String);

impl ToString for Error {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
