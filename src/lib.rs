use std::{marker::PhantomData, sync::Arc};

use anyhow::Error;
use poem::{endpoint::BoxEndpoint, Endpoint, IntoResponse, Middleware, Request, Response};

pub mod middleware;

pub struct NextError(Error);

type Result<T> = core::result::Result<T, NextError>;

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
        todo!()
    }
}

#[async_trait::async_trait]
pub trait NextMiddleware: Send + Sync + 'static {
    async fn handle(&self, req: Request, next: Next<'_>) -> Result<Response>;
}

pub trait NextExt: Endpoint<Output = Response> {
    fn wrap(self, next_middleware_group: &NextMiddlewareGroup) -> NextEndpoint<Self>
    where
        Self: Sized,
    {
        NextEndpoint {
            next_middleware: next_middleware_group.next_middleware.clone(),
            ep: Box::new(self),
            marker: PhantomData,
        }
    }
}

impl<T> NextExt for T where T: Endpoint<Output = Response> {}

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

#[derive(Default)]
pub struct NextMiddlewareGroup {
    next_middleware: Vec<Arc<dyn NextMiddleware>>,
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
