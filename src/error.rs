use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToonError {
    #[error("{message}")]
    Message { message: String },
}

pub type Result<T> = std::result::Result<T, ToonError>;

impl ToonError {
    #[must_use]
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message {
            message: message.into(),
        }
    }
}
