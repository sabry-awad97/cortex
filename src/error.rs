#[derive(Debug)]
pub enum CortexError {
    Io(std::io::Error),
    Syntax(String),
    Runtime(String),
}

impl std::error::Error for CortexError {}

impl From<std::io::Error> for CortexError {
    fn from(error: std::io::Error) -> Self {
        CortexError::Io(error)
    }
}

impl std::fmt::Display for CortexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CortexError::Io(e) => write!(f, "IO error: {}", e),
            CortexError::Syntax(e) => write!(f, "Syntax error: {}", e),
            CortexError::Runtime(e) => write!(f, "Runtime error: {}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, CortexError>;
