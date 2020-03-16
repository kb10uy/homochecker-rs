use homochecker_rs::{
    domain::Provider,
    repository::{AvatarRepository, RepositoryError, User, UserRepository},
};

use std::{collections::HashMap, sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone, Default)]
pub struct MockUserRepository {
    source: Arc<Mutex<Vec<User>>>,
}

#[allow(dead_code)]
impl MockUserRepository {
    pub async fn source(&self, source: Vec<User>) {
        let mut locked = self.source.lock().await;
        *locked = source;
    }

    pub fn as_source(&self) -> Arc<Mutex<Vec<User>>> {
        self.source.clone()
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn count_all(&self) -> Result<usize, RepositoryError> {
        Ok(self.source.lock().await.len())
    }

    async fn fetch_all(&self) -> Result<Vec<User>, RepositoryError> {
        Ok(self.source.lock().await.clone())
    }

    async fn fetch_by_screen_name(&self, screen_name: &str) -> Result<Vec<User>, RepositoryError> {
        let result = self
            .source
            .lock()
            .await
            .iter()
            .filter(|u| u.screen_name == screen_name)
            .cloned()
            .collect();
        Ok(result)
    }
}

#[derive(Clone, Default)]
pub struct MockAvatarRepository {
    source: Arc<Mutex<HashMap<Provider, Url>>>,
}

#[allow(dead_code)]
impl MockAvatarRepository {
    pub async fn source(&self, source: HashMap<Provider, Url>) {
        let mut locked = self.source.lock().await;
        *locked = source;
    }

    pub fn as_source(&self) -> Arc<Mutex<HashMap<Provider, Url>>> {
        self.source.clone()
    }
}

#[async_trait]
impl AvatarRepository for MockAvatarRepository {
    async fn get(&self, provider: &Provider) -> Result<Option<Url>, RepositoryError> {
        Ok(self.source.lock().await.get(provider).map(<_>::to_owned))
    }

    async fn save_cache(
        &self,
        provider: &Provider,
        url: &str,
        _age: Duration,
    ) -> Result<(), RepositoryError> {
        self.source
            .lock()
            .await
            .insert(provider.clone(), Url::parse(url)?);
        Ok(())
    }
}
