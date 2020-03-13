//! Contains validators related to service responses.

mod header;

use crate::homo::HomoServiceStatus;

use async_trait::async_trait;
use reqwest::Response;

/// Indicates that it validates `reqwest::Response`.
#[async_trait]
pub trait ValidateResponse {
    /// Validates the response.
    /// Returns `None` if any valid URL was found.
    async fn validate(response: &Response) -> Option<HomoServiceStatus>;
}

#[async_trait]
/// An extension trait that provides `ValidateResponse::validate`.
pub trait ValidateResponseExt {
    /// Validates the response.
    /// Returns `None` if any valid URL was found.
    async fn validate<V: ValidateResponse>(&self) -> Option<HomoServiceStatus>;
}

#[async_trait]
impl ValidateResponseExt for Response {
    async fn validate<V: ValidateResponse>(&self) -> Option<HomoServiceStatus> {
        V::validate(self).await
    }
}
