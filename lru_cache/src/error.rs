#[derive(Debug)]
pub enum CacheError {
    IOError(String),
    SerializationError(String),
    CapacityError(String),
}