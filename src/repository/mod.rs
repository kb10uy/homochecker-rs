//! Contains data repository.

mod avatar;
mod user;

use std::error::Error;

pub use self::avatar::AvatarRepository;
pub use self::user::{User, UserRepository};

/// Various error types in repository operations.
pub type RepositoryError = Box<dyn Error + Send + Sync>;

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
