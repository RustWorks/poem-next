use std::{marker::PhantomData, sync::Arc};

use crate::{Next, NextMiddleware, Result};
use poem::{endpoint::BoxEndpoint, Endpoint, Middleware, Request, Response};

#[derive(Default, Clone)]
pub struct NextMiddlewareGroup {
    pub(crate) next_middleware: Vec<Arc<dyn NextMiddleware>>,
}

impl From<Vec<Arc<dyn NextMiddleware>>> for NextMiddlewareGroup {
    fn from(next_middleware: Vec<Arc<dyn NextMiddleware>>) -> Self {
        NextMiddlewareGroup { next_middleware }
    }
}

impl NextMiddlewareGroup {
    pub fn push(&mut self, middleware: impl NextMiddleware) {
        self.next_middleware.push(Arc::new(middleware));
    }
}

impl<E> Middleware<E> for NextMiddlewareGroup
where
    E: Endpoint<Output = Response>,
{
    type Output = NextEndpoint<E>;

    fn transform(self, ep: E) -> Self::Output {
        NextEndpoint {
            next_middleware: self.next_middleware,
            ep: Box::new(ep),
            marker: PhantomData,
        }
    }
}

pub struct NextEndpoint<E> {
    next_middleware: Vec<Arc<dyn NextMiddleware>>,
    ep: BoxEndpoint<Response>,
    marker: PhantomData<E>,
}

#[async_trait::async_trait]
impl<E> Endpoint for NextEndpoint<E>
where
    E: Endpoint<Output = Response>,
{
    type Output = Result<Response>;

    async fn call(&self, req: Request) -> Self::Output {
        let next = Next {
            ep: &self.ep,
            next_middleware: &self.next_middleware,
        };

        next.run(req).await
    }
}
