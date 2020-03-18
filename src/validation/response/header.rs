use super::ValidateResponse;
use crate::domain::{HomoServiceStatus, HttpResponse};

use async_trait::async_trait;
use http::StatusCode;

/// Validates the response based on its response status and `Location` header.
pub enum ResponseHeaderValidator {}

#[async_trait]
impl ValidateResponse for ResponseHeaderValidator {
    async fn validate(response: &HttpResponse) -> Option<HomoServiceStatus> {
        match response.status {
            StatusCode::MOVED_PERMANENTLY
            | StatusCode::FOUND
            | StatusCode::SEE_OTHER
            | StatusCode::TEMPORARY_REDIRECT
            | StatusCode::PERMANENT_REDIRECT => {
                let location = match response.headers.get("location") {
                    Some(loc) => loc,
                    None => return Some(HomoServiceStatus::Invalid),
                };
                if location.starts_with("https://twitter.com/mpyw") {
                    Some(HomoServiceStatus::RedirectResponse)
                } else {
                    Some(HomoServiceStatus::Invalid)
                }
            }
            _ => None,
        }
    }
}
