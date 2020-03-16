//! Contains data repository.

use crate::domain::Provider;
use std::{time::Duration, error::Error};

use async_trait::async_trait;
use url::Url;

/// Various error types in repository operations.
pub type RepositoryError = Box<dyn Error + Send + Sync>;

/// Represents a record of `users`.
#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: i32,
    pub screen_name: String,
    pub service: String,
    pub url: String,
}

/// Represents the container which includes repositories.
pub trait Repositories
where
    Self: Sized + Clone + Send + Sync,
{
    /// The actual type for `UserRepository`.
    type User: UserRepository;

    /// The actual type for `UrlRepository`.
    type Avatar: AvatarRepository;

    /// Returns user repository.
    fn user(&self) -> Self::User;

    /// Returns URL repository.
    fn avatar(&self) -> Self::Avatar;
}

/// It can fetch users.
#[async_trait]
pub trait UserRepository
where
    Self: Sized + Clone + Send + Sync,
{
    /// Counts all records in `users`.
    async fn count_all(&self) -> Result<usize, RepositoryError>;

    /// Fetches all records from `users`.
    async fn fetch_all(&self) -> Result<Vec<User>, RepositoryError>;

    /// Fetches records with given screen_name.
    async fn fetch_by_screen_name(&self, screen_name: &str) -> Result<Vec<User>, RepositoryError>;
}

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
