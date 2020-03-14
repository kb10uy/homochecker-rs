//! Contains abstract domain model.

use crate::repository::User;
use std::{net::SocketAddr, time::Duration};

/// Represents a person who provides the homo service.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Provider {
    /// A Twitter user.
    Twitter(String),

    /// A Mastodon user.
    Mastodon { screen_name: String, domain: String },
}

/// Represents a person provides a homo service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomoService {
    /// The URL to this service.
    pub service_url: String,

    /// The screen name of this user.
    pub provider: Provider,

    /// The URL to the avatar image of this user.
    pub avatar_url: String,
}

/// Represents the status of the homo service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomoServiceStatus {
    /// The service returned a 301/302/308 response with specific `Location` header.
    RedirectResponse,

    /// The service returned a successful response which contains redirect meta element.
    RedirectContent,

    /// The service returned a successful response which contains just specific URL(s).
    LinkContent,

    /// The service returned a successful response which contains no valid URL.
    Invalid,

    /// The service returned an error.
    Error,
}

/// Represents the response information of homo service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomoServiceResponse {
    /// Status.
    pub status: HomoServiceStatus,

    /// The remote IP address.
    pub remote_address: Option<SocketAddr>,

    /// The response time.
    pub duration: Duration,

    /// The avatar URL of the provider.
    pub avatar_url: String,
}

impl Provider {
    /// Converts from entity text.
    pub fn from_entity(entity_sn: &str) -> Result<Provider, String> {
        let parts: Vec<_> = entity_sn.split('@').collect();
        match parts.len() {
            1 => Ok(Provider::Twitter(parts[0].into())),
            3 if parts[0] == "" => Ok(Provider::Mastodon {
                screen_name: parts[1].into(),
                domain: parts[2].into(),
            }),
            _ => Err("Invalid screen name expression".into()),
        }
    }

    /// Converts to entity text.
    pub fn to_entity_string(&self) -> String {
        match self {
            Provider::Twitter(s) => s.to_owned(),
            Provider::Mastodon {
                screen_name,
                domain,
            } => format!("@{}@{}", screen_name, domain),
        }
    }

    /// Converts to cache key.
    pub fn to_cache_key(&self) -> String {
        match self {
            Provider::Twitter(s) => format!("twitter:{}", s),
            Provider::Mastodon {
                screen_name,
                domain,
            } => format!("mastodon:@{}@{}", screen_name, domain),
        }
    }
}

impl HomoService {
    /// Builds `HomoService` from `User` entity.
    pub fn from_user(user: &User) -> Result<HomoService, String> {
        Ok(HomoService {
            provider: Provider::from_entity(&user.screen_name)?,
            avatar_url: user.avatar_url.to_owned(),
            service_url: user.service_url.to_owned(),
        })
    }
}
