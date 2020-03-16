//! Contais adapters for `UrlRepository`.

use crate::{
    data::Provider,
    repository::{AvatarRepository, RepositoryError},
    service::{AvatarService, ServiceError},
};
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use redis::{aio::Connection, AsyncCommands};
use reqwest::{Client, Response};
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct RedisAvatarAdapter(Arc<Mutex<Connection>>);

impl RedisAvatarAdapter {
    pub fn new(conn: Arc<Mutex<Connection>>) -> RedisAvatarAdapter {
        RedisAvatarAdapter(conn)
    }
}

#[async_trait]
impl AvatarRepository for RedisAvatarAdapter {
    async fn get(&self, provider: &Provider) -> Result<Option<Url>, RepositoryError> {
        let key = provider.to_cache_key();
        let mut locked = self.0.lock().await;
        let cached: Option<String> = locked.get(&key).await?;
        match cached {
            Some(url) => Ok(Some(Url::parse(&url)?)),
            None => Ok(None),
        }
    }

    async fn save_cache(
        &self,
        provider: &Provider,
        url: &str,
        age: Duration,
    ) -> Result<(), RepositoryError> {
        let key = provider.to_cache_key();
        let mut locked = self.0.lock().await;
        redis::cmd("SET")
            .arg(&key)
            .arg(url)
            .arg("EX")
            .arg(age.as_secs())
            .query_async(&mut *locked)
            .await?;

        Ok(())
    }
}

#[derive(Clone)]
struct WebAvatarService(Arc<Client>);

impl WebAvatarService {
    pub fn new(client: Arc<Client>) -> WebAvatarService {
        WebAvatarService(client)
    }
}

#[async_trait]
impl AvatarService for WebAvatarService {
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
