//! Contais adapters for `UrlRepository`.

use crate::{
    data::Provider,
    repository::{RepositoryError, UrlRepository},
};
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use redis::{aio::Connection, AsyncCommands};
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct RedisUrlAdapter(Arc<Mutex<Connection>>);

impl RedisUrlAdapter {
    pub fn new(conn: Arc<Mutex<Connection>>) -> RedisUrlAdapter {
        RedisUrlAdapter(conn)
    }
}

#[async_trait]
impl UrlRepository for RedisUrlAdapter {
    async fn get(&self, provider: &Provider) -> Result<Option<Url>, RepositoryError> {
        let key = provider.to_cache_key();
        let mut locked = self.0.lock().await;
        let cached: Option<String> = locked.get(&key).await?;
        match cached {
            Some(url) => Ok(Some(Url::parse(&url)?)),
            None => Ok(None),
        }
    }

    async fn set(
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
