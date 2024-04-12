#[derive(Debug)]
pub enum VErrors {
}

impl std::error::Error for VErrors {}

impl std::fmt::Display for VErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printable = match self {
            _ => "",
        };
        write!(f, "{}", printable)
    }
}
