use anyhow::Error;
use poem::{http::StatusCode, IntoResponse};

pub struct NextError(pub Error);

pub type Result<T> = core::result::Result<T, NextError>;

impl<T> From<T> for NextError
where
    T: Into<anyhow::Error>,
{
    fn from(e: T) -> Self {
        NextError(e.into())
    }
}

impl IntoResponse for NextError {
    fn into_response(self) -> poem::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into()
    }
}
