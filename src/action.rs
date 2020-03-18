//! Contains service logic related to `HomoService`.

use crate::{
    domain::{
        HomoService, HomoServiceResponse, HomoServiceStatus, HttpResponse, Provider,
        UnwrapOrWarnExt,
    },
    repository::{AvatarRepository, Repositories},
    service::{AvatarService, HomoRequestService, Services},
    validation::response::{ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt},
    Container,
};
use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use lazy_static::lazy_static;
use log::{info, warn};
use regex::Regex;
use serde_json::Value as JsonValue;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use url::Url;

pub type AvatarResolverAttached = (
    Vec<(HomoService, Receiver<Option<Url>>)>,
    HashMap<Provider, Sender<Option<Url>>>,
);

/// Requests to the service and validates its response whether contains appropriate link(s).
pub async fn request_service(
    deps: impl Container + 'static,
    service_url: Url,
) -> Result<HomoServiceResponse, Box<dyn Error + Send + Sync>> {
    let (response, duration) = deps.services().homo_request().request(&service_url).await?;

    let remote_address = response.remote_address;
    let status = validate_status(response).await;

    Ok(HomoServiceResponse {
        status,
        remote_address,
        duration,
    })
}

/// Validates the response from the service.
async fn validate_status(response: HttpResponse) -> HomoServiceStatus {
    match response.validate::<ResponseHeaderValidator>().await {
        Some(s) => s,
        None => response
            .validate::<ResponseHtmlValidator>()
            .await
            .unwrap_or(HomoServiceStatus::Invalid),
    }
}

pub async fn fetch_avatar(deps: impl Container + 'static, provider: Arc<Provider>) -> Option<Url> {
    lazy_static! {
        static ref REGEX_USER_AVATAR: Regex =
            Regex::new(r#"src=["'](https://[ap]bs\.twimg\.com/[^"']+)"#).unwrap();
    }

    let avatar_repo = deps.repositories().avatar();
    let avatar_srv = deps.services().avatar();

    // キャッシュ判定
    match avatar_repo.get(&provider).await {
        Ok(Some(url)) => return Some(url),
        Ok(None) => (),
        Err(e) => {
            warn!("Failed to access to Redis: {}", e);
        }
    }

    // 取得
    let fetched = match &*provider {
        Provider::Twitter(sn) => {
            let response = match avatar_srv.fetch_twitter(sn).await {
                Ok(res) => res,
                Err(e) => {
                    warn!("Failed to fetch twitter intent: {}", e);
                    return None;
                }
            };
            let html = response.body;
            if let Some(capture) = REGEX_USER_AVATAR.captures(&html) {
                Url::parse(&capture[1]).unwrap_or_warn("Invalid URL")
            } else {
                warn!("Avatar image not found in Twitter intent page: {}", sn);
                None
            }
        }
        Provider::Mastodon {
            screen_name,
            domain,
        } => {
            let response = match avatar_srv.fetch_mastodon(screen_name, domain).await {
                Ok(res) => res,
                Err(e) => {
                    warn!("Failed to fetch twitter intent: {}", e);
                    return None;
                }
            };
            let user = serde_json::from_str::<JsonValue>(&response.body)
                .unwrap_or_warn("Invalid Mastodon user JSON")?;
            if let JsonValue::String(s) = &user["icon"]["url"] {
                Url::parse(&s).unwrap_or_warn("Invalid URL")
            } else {
                let message = format!("user.icon.url was not string: {}", &user["icon"]["url"]);
                warn!("{}", message);
                None
            }
        }
    }?;

    // キャッシュ
    match avatar_repo
        .save_cache(&provider, &fetched.to_string(), Duration::from_secs(86400))
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
