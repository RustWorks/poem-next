use poem::http::header;

use crate::middleware::auth::Error;

use super::Scheme;

#[derive(Debug, Clone)]
pub struct Bearer;

pub struct Token(String);

#[async_trait::async_trait]
impl Scheme for Bearer {
    type Output = Token;

    fn header_name() -> poem::http::header::HeaderName {
        header::AUTHORIZATION
    }

    fn scheme() -> &'static str {
        "Bearer"
    }

    async fn parse(&self, req: &poem::Request) -> crate::Result<Self::Output> {
        let header_value = req
            .headers()
            .get(Self::header_name())
            .ok_or(Error::AUTHORIZATIONNotFound)?
            .to_str()
            .map_err(|e| Error::ToStr(e))?;

        let mut parts = header_value.splitn(2, ' ');

        match parts.next() {
            Some(scheme) if scheme == Self::scheme() => {}
            _ => return Err(Error::MissingScheme)?,
        }

        if let Some(output) = parts.next() {
            return Ok(Token(output.to_string()));
        } else {
            return Err(Error::MissingValue)?;
        }
    }
}
