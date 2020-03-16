pub mod homo;

use std::{error::Error, time::Duration};

use async_trait::async_trait;
use reqwest::Response;
use url::Url;

/// Various error types in service operations.
pub type ServiceError = Box<dyn Error + Send + Sync>;

/// Represents the container which includes services.
pub trait Services
where
    Self: Sized + Clone + Send + Sync,
{
    /// The actual type for `AvatarService`.
    type Avatar: AvatarService;

    /// The actual type for `HomoRequestService`.
    type HomoRequest: HomoRequestService;

    /// Returns avatar service.
    fn avatar(&self) -> Self::Avatar;

    /// Returns HomoService service.
    fn homo_request(&self) -> Self::HomoRequest;
}

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

/// Provides an interface to operations to check `HomoService`.
#[async_trait]
pub trait HomoRequestService
where
    Self: Sized + Send + Sync + Clone,
{
    async fn request(&self, service_url: &Url) -> Result<(Response, Duration), ServiceError>;
}
