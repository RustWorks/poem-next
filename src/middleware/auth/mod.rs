use poem::{
    http::header::{self, HeaderName},
    Request,
};

use crate::{NextMiddleware, Result};

mod bearer;
mod error;

pub use error::Error;

#[async_trait::async_trait]
pub trait Scheme: Send + Sync + 'static {
    type Output: Send + Sync + 'static;

    fn header_name() -> HeaderName {
        header::AUTHORIZATION
    }

    fn scheme() -> &'static str;

    async fn parse(&self, req: &Request) -> Result<Self::Output>;
}

pub struct Auth<S>(S);

#[async_trait::async_trait]
impl<S> Scheme for Auth<S>
where
    S: Scheme,
{
    type Output = <S as Scheme>::Output;

    fn header_name() -> HeaderName {
        <S as Scheme>::header_name()
    }

    fn scheme() -> &'static str {
        <S as Scheme>::scheme()
    }

    async fn parse(&self, req: &Request) -> Result<Self::Output> {
        self.0.parse(req).await
    }
}

#[async_trait::async_trait]
impl<S> NextMiddleware for Auth<S>
where
    S: Scheme,
{
    async fn handle(&self, mut req: Request, next: crate::Next<'_>) -> Result<poem::Response> {
        if let Some(_) = req.extensions().get::<<S as Scheme>::Output>() {
            next.run(req).await
        } else {
            let output = self.parse(&req).await?;
            req.extensions_mut().insert(output);
            next.run(req).await
        }
    }
}
