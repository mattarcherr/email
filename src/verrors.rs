#[derive(Debug)]
pub enum VError {
    Io(String),
}

impl std::error::Error for VError {}

impl std::fmt::Display for VError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            _ => "",
        };
        write!(f, "{}", printable)
    }
}

impl From<std::io::Error> for VError {
    fn from(err: std::io::Error) -> Self {
        VError::Io(err.to_string())
    }
}
