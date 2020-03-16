//! Contains user repository types.

use super::RepositoryError;

use async_trait::async_trait;

/// Represents a record of `users`.
pub struct User {
    pub id: i32,
    pub screen_name: String,
    pub service: String,
    pub url: String,
}

/// It can fetch users.
#[async_trait]
pub trait UserRepository {
    async fn count_all(&self) -> Result<usize, RepositoryError>;
    async fn fetch_all(&self) -> Result<Vec<User>, RepositoryError>;
    async fn fetch_by_screen_name(&self, screen_name: &str) -> Result<Vec<User>, RepositoryError>;
}
