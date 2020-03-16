//! Contains service logic related to `HomoService`.

use crate::{
    data::{HomoService, HomoServiceResponse, HomoServiceStatus, Provider, UnwrapOrWarnExt},
    repository::UrlRepository,
    validation::response::{ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt},
};
use std::{
    collections::HashMap,
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use log::{info, warn};
use regex::Regex;
use reqwest::{Client, Error as ReqwestError, Response};
use serde_json::Value as JsonValue;
use tokio::{
    join,
    sync::broadcast::{channel, Receiver, Sender},
};
use url::Url;

pub type AvatarResolverAttached = (
    Vec<(HomoService, Receiver<Option<Url>>)>,
    HashMap<Provider, Sender<Option<Url>>>,
);

/// Requests to the service and validates its response whether contains appropriate link(s).
pub async fn request_service(
    client: Client,
    service: Arc<HomoService>,
    mut avatar_url_receiver: Receiver<Option<Url>>,
) -> Result<HomoServiceResponse, Box<dyn Error + Send + Sync>> {
    let request = client.get(&service.service_url[..]);
    let start_at = Instant::now();
    let response = request.send().await;

    let duration = start_at.elapsed();
    let remote_address = response.as_ref().map(|r| r.remote_addr()).ok().flatten();

    // アバター URL と リダイレクト判定は並行
    let (avatar_url, status) = join!(avatar_url_receiver.recv(), validate_status(response));

    Ok(HomoServiceResponse {
        status,
        remote_address,
        duration,
        avatar_url: avatar_url?,
    })
}

/// Validates the response from the service.
async fn validate_status(response: Result<Response, ReqwestError>) -> HomoServiceStatus {
    let response = match response {
        Ok(r) => r,
        Err(_) => return HomoServiceStatus::Error,
    };
    match response.validate::<ResponseHeaderValidator>().await {
        Some(s) => s,
        None => response
            .into_validate::<ResponseHtmlValidator>()
            .await
            .unwrap_or(HomoServiceStatus::Invalid),
    }
}

pub async fn fetch_avatar(
    url_repo: impl UrlRepository,
    client: Client,
    provider: Arc<Provider>,
) -> Option<Url> {
    match url_repo.get(&provider).await {
        Ok(Some(url)) => return Some(url),
        Ok(None) => (),
        Err(e) => {
            warn!("Failed to access to Redis: {}", e);
        }
    }

    let fetched = match &*provider {
        Provider::Twitter(sn) => fetch_twitter_avatar(client, &sn).await,
        Provider::Mastodon {
            screen_name,
            domain,
        } => fetch_mastodon_avatar(client, &screen_name, &domain).await,
    }?;

    match url_repo
        .set(&provider, &fetched.to_string(), Duration::from_secs(86400))
        .await
    {
        Ok(()) => {
            info!("Cached `{:?}`: {}", &provider, fetched);
        }
        Err(e) => {
            warn!("Failed to access to Redis: {}", e);
        }
    };

    Some(fetched)
}

/// Attaches broadcasters to services.
/// Second tuple element is a map from screen name to `Sender`.
pub fn attach_avatar_resolver(
    services: impl IntoIterator<Item = HomoService>,
) -> AvatarResolverAttached {
    let mut attached = vec![];
    let mut txmap: HashMap<Provider, Sender<Option<Url>>> = HashMap::new();
    for service in services {
        let sn = &service.provider;
        if let Some(tx) = txmap.get(&sn) {
            attached.push((service, tx.subscribe()));
        } else {
            let (tx, rx) = channel(4);
            txmap.insert(sn.clone(), tx);
            attached.push((service, rx));
        }
    }

    (attached, txmap)
}

/// Fetches specific Mastodon user avatar URL from Web.
async fn fetch_twitter_avatar(client: Client, screen_name: &str) -> Option<Url> {
    lazy_static! {
        static ref REGEX_USER_AVATAR: Regex =
            Regex::new(r#"src=["'](https://[ap]bs\.twimg\.com/[^"']+)"#).unwrap();
    }

    let request = client.get(&format!(
        "https://twitter.com/intent/user?screen_name={}",
        screen_name
    ));
    let html = request
        .send()
        .await
        .unwrap_or_warn("Failed to fetch Twitter intent")?
        .text()
        .await
        .unwrap_or_warn("Body error")?;

    if let Some(capture) = REGEX_USER_AVATAR.captures(&html) {
        Url::parse(&capture[1]).unwrap_or_warn("Invalid URL")
    } else {
        warn!(
            "Avatar image not found in Twitter intent page: {}",
            screen_name
        );
        None
    }
}

/// Fetches specific Mastodon user avatar URL from Web.
async fn fetch_mastodon_avatar(client: Client, screen_name: &str, domain: &str) -> Option<Url> {
    let request = client
        .get(&format!("https://{}/users/{}.json", domain, screen_name))
        .header("Accept", "application/json");

    let user = request
        .send()
        .await
        .unwrap_or_warn("Failed to fetch Mastodon user JSON")?
        .json::<JsonValue>()
        .await
        .unwrap_or_warn("Invalid Mastodon user JSON")?;

    if let JsonValue::String(s) = &user["icon"]["url"] {
        Url::parse(&s).unwrap_or_warn("Invalid URL")
    } else {
        let message = format!("user.icon.url was not string: {}", &user["icon"]["url"]);
        warn!("{}", message);
        None
    }
}
