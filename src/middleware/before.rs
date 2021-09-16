use poem::{Request, Response};
use std::future::Future;

use crate::NextMiddleware;

pub struct Before<F>(F);

impl<F> Clone for Before<F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Before(self.0.clone())
    }
}

#[async_trait::async_trait]
impl<F, Fut> NextMiddleware for Before<F>
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Request> + Send + 'static,
{
    async fn handle(&self, req: Request, next: crate::Next<'_>) -> crate::Result<Response> {
        let req = (self.0)(req).await;

        next.run(req).await
    }
}
