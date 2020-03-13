//! Contains validators related to service responses.

mod header;
mod html;

pub use header::ResponseHeaderValidator;
pub use html::ResponseHtmlValidator;

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

/// Indicates that it validates **moved** `reqwest::Response`.
#[async_trait]
pub trait IntoValidateResponse {
    /// Validates the response.
    /// Returns `None` if any valid URL was found.
    async fn into_validate(response: Response) -> Option<HomoServiceStatus>;
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

    async fn into_validate<IV: IntoValidateResponse>(self) -> Option<HomoServiceStatus>;
}

#[async_trait]
impl ValidateResponseExt for Response {
    async fn validate<V: ValidateResponse>(&self) -> Option<HomoServiceStatus> {
        V::validate(self).await
    }

    async fn into_validate<IV: IntoValidateResponse>(self) -> Option<HomoServiceStatus> {
        IV::into_validate(self).await
    }
}
