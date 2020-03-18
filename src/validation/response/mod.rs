//! Contains validators related to service responses.

mod header;
mod html;

pub use self::{header::ResponseHeaderValidator, html::ResponseHtmlValidator};

use crate::domain::{HomoServiceStatus, HttpResponse};

use async_trait::async_trait;

/// Indicates that it validates `HttpResponse`.
#[async_trait]
pub trait ValidateResponse {
    /// Validates the response.
    /// Returns `None` if any valid URL was found.
    async fn validate(response: &HttpResponse) -> Option<HomoServiceStatus>;
}

#[async_trait]
/// An extension trait that provides `ValidateResponse::validate`.
pub trait ValidateResponseExt
where
    Self: Sized,
{
    /// Validates the response.
    /// Returns `None` if any valid URL was found.
    async fn validate<V: ValidateResponse>(&self) -> Option<HomoServiceStatus>;
}

#[async_trait]
impl ValidateResponseExt for HttpResponse {
    async fn validate<V: ValidateResponse>(&self) -> Option<HomoServiceStatus> {
        V::validate(self).await
    }
}
