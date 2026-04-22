use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Error retreiving timestamp: {0}")]
    NoTime(String),
}
