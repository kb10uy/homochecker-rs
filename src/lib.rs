//! Contains all functionalities for homochecker-rs.

pub mod adapter;
pub mod api;
pub mod data;
pub mod repository;
pub mod service;
pub mod validation;

use self::{repository::Repositories, service::Services};

/// Represents the container of dependencies.
pub trait Container
where
    Self: Sized + Clone + Send + Sync,
{
    /// The actual type of `Repositories`.
    type Repositories: Repositories;

    /// The actual type of `Services`.
    type Services: Services;

    /// Returns repositories.
    fn repositories(&self) -> Self::Repositories;

    /// Returns services.
    fn services(&self) -> Self::Services;
}
