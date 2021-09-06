use std::convert::TryInto;

use poem::{
    http::{header::HeaderName, HeaderValue},
    Request, Response,
};

use crate::NextMiddleware;

enum Action {
    Override(HeaderName, HeaderValue),
    Append(HeaderName, HeaderValue),
}

/// Middleware for override/append headers to response.
#[derive(Default)]
pub struct SetHeader {
    actions: Vec<Action>,
}

impl SetHeader {
    /// Create new `SetHeader` middleware.
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Inserts a header to response.
    ///
    /// If a previous value exists for the same header, it is
    /// removed and replaced with the new header value.
    #[must_use]
    pub fn overriding<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        let key = key.try_into();
        let value = value.try_into();
        if let (Ok(key), Ok(value)) = (key, value) {
            self.actions.push(Action::Override(key, value));
        }
        self
    }

    /// Appends a header to response.
    ///
    /// If previous values exist, the header will have multiple values.
    #[must_use]
    pub fn appending<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
    {
        let key = key.try_into();
        let value = value.try_into();
        if let (Ok(key), Ok(value)) = (key, value) {
            self.actions.push(Action::Append(key, value));
        }
        self
    }
}

#[async_trait::async_trait]
impl NextMiddleware for SetHeader {
    async fn handle(&self, req: Request, next: crate::Next<'_>) -> crate::Result<Response> {
        let mut resp = next.run(req).await?;
        let headers = resp.headers_mut();
        for action in &self.actions {
            match action {
                Action::Override(name, value) => {
                    headers.insert(name.clone(), value.clone());
                }
                Action::Append(name, value) => {
                    headers.append(name.clone(), value.clone());
                }
            }
        }

        Ok(resp)
    }
}
