use super::ValidateResponse;
use crate::domain::HomoServiceStatus;

use async_trait::async_trait;
use reqwest::{Response, StatusCode};

/// Validates the response based on its response status and `Location` header.
pub enum ResponseHeaderValidator {}

#[async_trait]
impl ValidateResponse for ResponseHeaderValidator {
    async fn validate(response: &Response) -> Option<HomoServiceStatus> {
        match response.status() {
            StatusCode::MOVED_PERMANENTLY
            | StatusCode::FOUND
            | StatusCode::TEMPORARY_REDIRECT
            | StatusCode::PERMANENT_REDIRECT => {
                let location = match response.headers().get("location") {
                    Some(loc) => loc,
                    None => return Some(HomoServiceStatus::Invalid),
                };
                match location.to_str() {
                    Ok(loc_str) => {
                        if loc_str.starts_with("https://twitter.com/mpyw") {
                            Some(HomoServiceStatus::RedirectResponse)
                        } else {
                            Some(HomoServiceStatus::Invalid)
                        }
                    }
                    Err(_) => Some(HomoServiceStatus::Invalid),
                }
            }
            _ => None,
        }
    }
}
