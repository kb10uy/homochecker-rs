use crate::data::{HomoService, HomoServiceResponse, HomoServiceStatus, Provider};

use serde::{Deserialize, Serialize};

/// Response format for `GET /check/*`.
#[derive(Debug, Deserialize)]
pub enum CheckResponseFormat {
    #[serde(rename = "sse")]
    ServerSentEvent,
    #[serde(rename = "json")]
    Json,
}

/// Represents a data object of query parameter of `GET /check/*`.
#[derive(Debug, Deserialize)]
pub struct CheckQueryParameter {
    pub format: Option<CheckResponseFormat>,
}

/// Represents a data object of 'initialize' event in `GET /check`.
#[derive(Debug, Serialize)]
pub struct CheckEventInitializeData {
    pub count: usize,
}

/// Represents `homo` property of the data object of 'response' event in `GET /check`.
#[derive(Debug, Serialize)]
pub struct CheckEventResponseDataHomo {
    pub screen_name: String,
    pub service: String,
    pub icon: String,
    pub url: String,
    pub display_url: String,
    pub secure: bool,
}

/// Represents a data object of 'response' event in `GET /check`.
#[derive(Debug, Serialize)]
pub struct CheckEventResponseData {
    pub homo: CheckEventResponseDataHomo,
    pub status: String,
    pub ip: Option<String>,
    pub duration: f64,
}

impl CheckEventResponseData {
    pub fn build(service: &HomoService, response: &HomoServiceResponse) -> CheckEventResponseData {
        // TODO: display_ur; を整形
        CheckEventResponseData {
            homo: CheckEventResponseDataHomo {
                screen_name: match &service.provider {
                    Provider::Twitter(sn) => sn.to_owned(),
                    Provider::Mastodon {
                        screen_name,
                        domain,
                    } => format!("@{}@{}", screen_name, domain),
                },
                service: match &service.provider {
                    Provider::Twitter(_) => "twitter",
                    Provider::Mastodon { .. } => "mastodon",
                }
                .into(),
                icon: response.avatar_url.to_owned(),
                url: service.service_url.to_owned(),
                display_url: service.service_url.to_owned(),
                secure: service.service_url.starts_with("https"),
            },
            status: match response.status {
                HomoServiceStatus::RedirectResponse | HomoServiceStatus::RedirectContent => "OK",
                HomoServiceStatus::LinkContent => "CONTAINS",
                HomoServiceStatus::Invalid => "WRONG",
                HomoServiceStatus::Error => "ERROR",
            }
            .into(),
            ip: response.remote_address.map(|a| a.ip().to_string()),
            duration: response.duration.as_secs_f64(),
        }
    }
}
