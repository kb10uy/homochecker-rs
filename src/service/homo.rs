use crate::{
    data::{HomoService, HomoServiceResponse, HomoServiceStatus},
    validation::response::{ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt},
};
use std::{sync::Arc, time::Instant};

use reqwest::{Client, Error as ReqwestError};

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
