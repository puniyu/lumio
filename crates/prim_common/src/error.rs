use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Decode failed")]
    Decode,
    #[error("Encode failed")]
    Encode,
}