#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InsufficientBytes,
    InvalidEncoding,
    InvalidVersion,
}
