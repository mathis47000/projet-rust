#[derive(Debug)]
pub enum CacheError {
    SerializationError,
    DeserializationError,
    IOError(std::io::Error),
}