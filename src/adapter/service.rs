//! Contais adapters for `UserRepository`.

use homochecker_rs::{
    domain::HttpResponse,
    service::{
        AvatarService as AvatarServiceInterface, HomoRequestService as HomoRequestServiceInterface,
        ServiceError,
    },
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use reqwest::Client;
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
    async fn fetch_twitter(&self, screen_name: &str) -> Result<HttpResponse, ServiceError> {
        let client = &self.0;
        let request = client.get(&format!(
            "https://twitter.com/intent/user?screen_name={}",
            screen_name
        ));

        let response = request.send().await?;
        let status = response.status();
        let remote_address = response.remote_addr();
        let mut headers = HashMap::new();
        for (k, v) in response.headers() {
            headers.insert(k.as_str().to_owned(), v.to_str()?.to_owned());
        }
        Ok(HttpResponse {
            status,
            remote_address,
            headers,
            body: response.text().await?,
        })
    }

    async fn fetch_mastodon(
        &self,
        screen_name: &str,
        domain: &str,
    ) -> Result<HttpResponse, ServiceError> {
        let client = &self.0;
        let request = client
            .get(&format!("https://{}/users/{}.json", domain, screen_name))
            .header("Accept", "application/json");

        let response = request.send().await?;
        let status = response.status();
        let remote_address = response.remote_addr();
        let mut headers = HashMap::new();
        for (k, v) in response.headers() {
            headers.insert(k.as_str().to_owned(), v.to_str()?.to_owned());
        }
        Ok(HttpResponse {
            status,
            remote_address,
            headers,
            body: response.text().await?,
        })
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
    async fn request(&self, service_url: &Url) -> Result<(HttpResponse, Duration), ServiceError> {
        let client = &self.0;
        let start = Instant::now();
        let response = client.get(&service_url[..]).send().await?;
        let duration = start.elapsed();

        let status = response.status();
        let remote_address = response.remote_addr();
        let mut headers = HashMap::new();
        for (k, v) in response.headers() {
            headers.insert(k.as_str().to_owned(), v.to_str()?.to_owned());
        }
        Ok((
            HttpResponse {
                status,
                remote_address,
                headers,
                body: response.text().await?,
            },
            duration,
        ))
    }
}
