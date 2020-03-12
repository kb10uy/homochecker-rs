//! Contains abstract domain model.

use std::{
    time::Duration,
    net::SocketAddr,
};

/// Represents a person provides a homo service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomoService {
    /// The URL to this service.
    pub service_url: String,

    /// The screen name of this user.
    pub screen_name: String,

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
    status: HomoServiceStatus,

    /// The remote IP address.
    remote_address: Option<SocketAddr>,

    /// The response time.
    duration: Duration,
}
