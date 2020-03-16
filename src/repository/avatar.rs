//! Contains URL cache repository types.

use super::RepositoryError;
use crate::data::Provider;
use std::time::Duration;

use async_trait::async_trait;
use url::Url;

/// It can fetch avatar URL with cache.
#[async_trait]
pub trait AvatarRepository
where
    Self: Sized + Clone + Send + Sync,
{
    /// Gets URL cache.
    async fn get(&self, provider: &Provider) -> Result<Option<Url>, RepositoryError>;

    /// Sets URL cache with expiration age.
    async fn save_cache(
        &self,
        provider: &Provider,
        url: &str,
        age: Duration,
    ) -> Result<(), RepositoryError>;
}
