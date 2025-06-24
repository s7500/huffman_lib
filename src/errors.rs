#[derive(Debug)]
pub enum CustomError {
    FileNotFound(String),
    CharNotFound,
    InvalidInput,
    CompressionFailed,
    DecompressionFailed,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomError::FileNotFound(path) => write!(f, "File not found at {}", path),
            CustomError::InvalidInput => write!(f, "Invalid input"),
            CustomError::CompressionFailed => write!(f, "Compression failed"),
            CustomError::DecompressionFailed => write!(f, "Decompression failed"),
            CustomError::CharNotFound => write!(f, "Character not found"),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => CustomError::FileNotFound(err.to_string()),
            _ => CustomError::CompressionFailed,
        }
    }
}
