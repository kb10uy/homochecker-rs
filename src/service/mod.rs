pub mod homo;

use std::error::Error;

use async_trait::async_trait;
use reqwest::Response;

type ServiceError = Box<dyn Error + Send + Sync>;

/// Provides an interface to operations to fetch avatar URL.
#[async_trait]
pub trait AvatarService
where
    Self: Sized + Send + Sync + Clone,
{
    async fn fetch_twitter(&self, screen_name: &str) -> Result<Response, ServiceError>;
    async fn fetch_mastodon(
        &self,
        screen_name: &str,
        domain: &str,
    ) -> Result<Response, ServiceError>;
}
