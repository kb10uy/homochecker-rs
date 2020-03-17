use super::Ambox;
use homochecker_rs::service::{AvatarService, HomoRequestService, ServiceError};
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use reqwest::Response;
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct MockAvatarService {
    for_twitter: Ambox<dyn Fn(&str) -> Response + Send + Sync>,
    for_mastodon: Ambox<dyn Fn(&str, &str) -> Response + Send + Sync>,
}

impl Default for MockAvatarService {
    fn default() -> MockAvatarService {
        MockAvatarService::new()
    }
}

#[allow(dead_code)]
impl MockAvatarService {
    pub fn new() -> MockAvatarService {
        MockAvatarService {
            for_twitter: Arc::new(Mutex::new(Box::new(|_| todo!()))),
            for_mastodon: Arc::new(Mutex::new(Box::new(|_, _| todo!()))),
        }
    }

    pub fn for_twitter(&self) -> Ambox<dyn Fn(&str) -> Response + Send + Sync> {
        self.for_twitter.clone()
    }

    pub fn for_mastodon(&self) -> Ambox<dyn Fn(&str, &str) -> Response + Send + Sync> {
        self.for_mastodon.clone()
    }
}

#[async_trait]
impl AvatarService for MockAvatarService {
    async fn fetch_twitter(&self, screen_name: &str) -> Result<Response, ServiceError> {
        let function = self.for_twitter.lock().await;
        Ok(function(screen_name))
    }

    async fn fetch_mastodon(
        &self,
        screen_name: &str,
        domain: &str,
    ) -> Result<Response, ServiceError> {
        let function = self.for_mastodon.lock().await;
        Ok(function(screen_name, domain))
    }
}

#[derive(Clone)]
pub struct MockHomoRequestService {
    source: Ambox<dyn Fn() -> (Response, Duration) + Send + Sync>,
}

impl Default for MockHomoRequestService {
    fn default() -> MockHomoRequestService {
        MockHomoRequestService::new()
    }
}

#[allow(dead_code)]
impl MockHomoRequestService {
    pub fn new() -> MockHomoRequestService {
        MockHomoRequestService {
            source: Arc::new(Mutex::new(Box::new(|| todo!()))),
        }
    }

    pub fn source(&self) -> Ambox<dyn Fn() -> (Response, Duration) + Send + Sync> {
        self.source.clone()
    }
}

#[async_trait]
impl HomoRequestService for MockHomoRequestService {
    async fn request(&self, _: &Url) -> Result<(Response, Duration), ServiceError> {
        let function = self.source.lock().await;
        Ok(function())
    }
}
