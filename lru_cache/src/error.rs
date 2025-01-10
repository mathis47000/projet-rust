#[derive(Debug)]
pub enum CacheError {
    IoError(String),
    SerializationError(String),
    CapacityError(String),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::IoError(msg) => write!(f, "IO Error: {}", msg),
            CacheError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            CacheError::CapacityError(msg) => write!(f, "Capacity Error: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {}