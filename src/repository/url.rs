//! Contains URL cache repository types.

use super::RepositoryError;
use crate::data::Provider;
use std::time::Duration;

use async_trait::async_trait;
use url::Url;

/// It can fetch URL cache.
#[async_trait]
pub trait UrlRepository {
    /// Gets URL cache.
    async fn get(&self, provider: &Provider) -> Result<Option<Url>, RepositoryError>;

    /// Sets URL cache with expiration age.
    async fn set(provider: &Provider, url: &str, age: Duration) -> Result<(), RepositoryError>;
}
