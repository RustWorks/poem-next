use poem::http::header::ToStrError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("AUTHORIZATIONNotFound")]
    AUTHORIZATIONNotFound,
    #[error("ToStr: {0}")]
    ToStr(ToStrError),
    #[error("MissingScheme")]
    MissingScheme,
    #[error("MissingValue")]
    MissingValue,
}
