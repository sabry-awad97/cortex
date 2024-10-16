#[derive(Debug)]
pub enum CortexError {
    Io(std::io::Error),
    Runtime(String),
    Syntax { message: String, position: usize },
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
            CortexError::Syntax { message, position } => {
                write!(f, "Syntax error: {} at position {}", message, position)
            }
            CortexError::Runtime(e) => write!(f, "Runtime error: {}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, CortexError>;
