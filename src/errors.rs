#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_error_display() {
        let error = CustomError::FileNotFound("test.txt".to_string());
        assert_eq!(format!("{}", error), "File not found at test.txt");

        let error = CustomError::InvalidInput;
        assert_eq!(format!("{}", error), "Invalid input");

        let error = CustomError::CompressionFailed;
        assert_eq!(format!("{}", error), "Compression failed");

        let error = CustomError::DecompressionFailed;
        assert_eq!(format!("{}", error), "Decompression failed");

        let error = CustomError::CharNotFound;
        assert_eq!(format!("{}", error), "Character not found");
    }

    #[test]
    fn test_from_io_error_not_found() {
        let io_error = Error::new(ErrorKind::NotFound, "File not found");
        let custom_error = CustomError::from(io_error);

        match custom_error {
            CustomError::FileNotFound(_) => (),
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_from_io_error_other() {
        let io_error = Error::new(ErrorKind::PermissionDenied, "Permission denied");
        let custom_error = CustomError::from(io_error);

        assert_eq!(custom_error, CustomError::CompressionFailed);
    }

    #[test]
    fn test_error_trait() {
        let error = CustomError::InvalidInput;
        let _: &dyn std::error::Error = &error;
    }
}
