//! Contais adapters for `UserRepository`.

use homochecker_rs::service::{
    AvatarService as AvatarServiceInterface, HomoRequestService as HomoRequestServiceInterface,
    ServiceError,
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use reqwest::{Client, Response};
use url::Url;

#[derive(Clone)]
pub struct AvatarService(Arc<Client>);

impl AvatarService {
    pub fn new(client: Arc<Client>) -> AvatarService {
        AvatarService(client)
    }
}

#[async_trait]
impl AvatarServiceInterface for AvatarService {
    async fn fetch_twitter(&self, screen_name: &str) -> Result<Response, ServiceError> {
        let client = &self.0;
        let request = client.get(&format!(
            "https://twitter.com/intent/user?screen_name={}",
            screen_name
        ));

        Ok(request.send().await?)
    }

    async fn fetch_mastodon(
        &self,
        screen_name: &str,
        domain: &str,
    ) -> Result<Response, ServiceError> {
        let client = &self.0;
        let request = client
            .get(&format!("https://{}/users/{}.json", domain, screen_name))
            .header("Accept", "application/json");

        Ok(request.send().await?)
    }
}

#[derive(Clone)]
pub struct HomoRequestService(Arc<Client>);

impl HomoRequestService {
    pub fn new(client: Arc<Client>) -> HomoRequestService {
        HomoRequestService(client)
    }
}

#[async_trait]
impl HomoRequestServiceInterface for HomoRequestService {
    async fn request(&self, service_url: &Url) -> Result<(Response, Duration), ServiceError> {
        let client = &self.0;
        let start = Instant::now();
        let response = client.get(&service_url[..]).send().await?;
        let duration = start.elapsed();

        Ok((response, duration))
    }
}
