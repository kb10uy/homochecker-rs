use super::ValidateResponse;
use crate::domain::{HomoServiceStatus, HttpResponse};

use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_HTML_META: Regex = Regex::new(r#"<meta\s+([^>]+)\s*>"#).unwrap();
    static ref REGEX_HTML_ATTR: Regex = Regex::new(r#"([a-zA-Z0-9\-]+)="([^"]+)""#).unwrap();
    static ref REGEX_TARGET_URL: Regex = Regex::new(r#"https://twitter\.com/mpyw"#).unwrap();
}

/// Validates the response based on its HTML response body.
pub enum ResponseHtmlValidator {}

#[async_trait]
impl ValidateResponse for ResponseHtmlValidator {
    async fn validate(response: &HttpResponse) -> Option<HomoServiceStatus> {
        for meta in REGEX_HTML_META.captures_iter(&response.body) {
            let mut http_equiv = false;
            let mut content = false;
            for attr in REGEX_HTML_ATTR.captures_iter(&meta[1]) {
                match &attr[1] {
                    "http-equiv" => {
                        http_equiv |= &attr[2].to_lowercase() == "refresh";
                    }
                    "content" => {
                        content |= REGEX_TARGET_URL.is_match(&attr[2]);
                    }
                    _ => continue,
                }
            }
            if http_equiv && content {
                return Some(HomoServiceStatus::RedirectContent);
            }
        }

        if REGEX_TARGET_URL.is_match(&response.body) {
            Some(HomoServiceStatus::LinkContent)
        } else {
            None
        }
    }
}
