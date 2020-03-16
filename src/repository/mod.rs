//! Contains data repository.

mod url;
mod user;

use std::error::Error;

pub use self::url::UrlRepository;
pub use self::user::{User, UserRepository};

/// Various error types in repository operations.
pub type RepositoryError = Box<dyn Error + Send + Sync>;

/// Represents the container which includes repositories.
pub trait Repositories {
    /// The actual type for `UserRepository`.
    type User: UserRepository;

    /// The actual type for `UrlRepository`.
    type Url: UrlRepository;

    /// Returns user repository.
    fn user() -> Self::User;

    /// Returns URL repository.
    fn url() -> Self::Url;
}
