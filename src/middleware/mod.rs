use std::sync::Arc;

use poem::{endpoint::BoxEndpoint, Request, Response};

use crate::Result;

pub mod auth;
mod set_header;

pub use set_header::SetHeader;

#[async_trait::async_trait]
pub trait NextMiddleware: Send + Sync + 'static {
    async fn handle(&self, req: Request, next: Next<'_>) -> Result<Response>;
}

#[async_trait::async_trait]
impl<T> NextMiddleware for Box<T>
where
    T: NextMiddleware,
{
    async fn handle(&self, req: Request, next: Next<'_>) -> Result<Response> {
        self.handle(req, next).await
    }
}

pub struct Next<'a> {
    pub(crate) next_middleware: &'a [Arc<dyn NextMiddleware>],
    pub(crate) ep: &'a BoxEndpoint<Response>,
}

impl Next<'_> {
    pub async fn run(mut self, req: Request) -> Result<Response> {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(req, self).await
        } else {
            Ok(self.ep.call(req).await)
        }
    }
}
