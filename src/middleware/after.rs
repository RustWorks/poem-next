use poem::{Request, Response};
use std::future::Future;

use crate::{NextMiddleware, Result};

pub struct After<F>(F);

impl<F> After<F> {
    pub fn new(inner: F) -> Self {
        After(inner)
    }
}

impl<F> Clone for After<F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        After(self.0.clone())
    }
}

#[async_trait::async_trait]
impl<F, Fut> NextMiddleware for After<F>
where
    F: Fn(Response) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response>> + Send + 'static,
{
    async fn handle(&self, req: Request, next: crate::Next<'_>) -> crate::Result<Response> {
        let resp = next.run(req).await?;

        (self.0)(resp).await
    }
}
