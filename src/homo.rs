//! Contains abstract domain model.

use std::{net::SocketAddr, time::Duration};

use serde::{ser::SerializeMap, Serialize, Serializer};

/// Represents a person who provides the homo service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provider {
    /// A Twitter user.
    Twitter(String),

    /// A Mastodon user.
    Mastodon {
        screen_name: String,
        domain: String,
    }
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
}
