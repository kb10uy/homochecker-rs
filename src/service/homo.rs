use crate::{
    data::{HomoService, HomoServiceResponse, HomoServiceStatus, Provider},
    validation::response::{ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt},
};
use std::{error::Error, sync::Arc, time::Instant};

use lazy_static::lazy_static;
use log::warn;
use regex::Regex;
use reqwest::{Client, Error as ReqwestError};
use serde_json::Value as JsonValue;

/// Requests to the service and validates its response whether contains appropriate link(s).
pub async fn request_service(
    client: Client,
    service: Arc<HomoService>,
) -> Result<HomoServiceResponse, ReqwestError> {
    let request = client.get(&service.service_url);
    let start_at = Instant::now();
    let response = request.send().await;

    let duration = start_at.elapsed();
    let remote_address = response.as_ref().map(|r| r.remote_addr()).ok().flatten();
    let status = match response {
        Ok(r) => match r.validate::<ResponseHeaderValidator>().await {
            Some(s) => s,
            None => r
                .into_validate::<ResponseHtmlValidator>()
                .await
                .unwrap_or(HomoServiceStatus::Invalid),
        },
        Err(_) => HomoServiceStatus::Error,
    };

    Ok(HomoServiceResponse {
        status,
        remote_address,
        duration,
    })
}

async fn fetch_avatar(client: Client, provider: Arc<Provider>) -> Result<String, Box<dyn Error>> {
    // TODO: キャッシュする
    let key = provider.to_entity_string();

    match &*provider {
        Provider::Twitter(sn) => fetch_twutter_avatar(client, &sn).await,
        Provider::Mastodon {
            screen_name,
            domain,
        } => fetch_mastodon_avatar(client, &screen_name, &domain).await,
    }
}

/// Fetches specific Mastodon user avatar URL from Web.
async fn fetch_twutter_avatar(client: Client, screen_name: &str) -> Result<String, Box<dyn Error>> {
    lazy_static! {
        static ref REGEX_USER_AVATAR: Regex =
            Regex::new(r#"src=["'](https://[ap]bs\.twimg\.com/[^"']+)"#).unwrap();
    }

    let request = client.get(&format!(
        "https://twitter.com/intent/user?screen_name={}",
        screen_name
    ));
    let html = request.send().await?.text().await?;

    if let Some(capture) = REGEX_USER_AVATAR.captures(&html) {
        Ok(capture[1].into())
    } else {
        let message = format!(
            "Avatar image not found in Twitter intent page: {}",
            screen_name
        );
        warn!("{}", message);
        Err(message.into())
    }
}

/// Fetches specific Mastodon user avatar URL from Web.
async fn fetch_mastodon_avatar(
    client: Client,
    screen_name: &str,
    domain: &str,
) -> Result<String, Box<dyn Error>> {
    let request = client
        .get(&format!("https://{}/users/{}.json", domain, screen_name))
        .header("Accept", "application/json");

    let user = request.send().await?.json::<JsonValue>().await?;
    let avatar_url = &user["icon"]["url"];
    match avatar_url {
        JsonValue::String(s) => Ok(s.to_owned()),
        _ => {
            let message = format!("user.icon.url was not string: {}", avatar_url);
            warn!("{}", message);
            Err(message.into())
        }
    }
}
