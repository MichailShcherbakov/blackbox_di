#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<'a>(msg: &'a str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TraitCastingError: {}", &self.msg.as_str())
    }
}
