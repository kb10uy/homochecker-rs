use futures::{channel::mpsc::channel as futures_channel, prelude::*};
use reqwest::{redirect::Policy as RedirectPolicy, Client};
use tokio::spawn;
use warp::Reply;

use crate::homo::{HomoService, HomoServiceResponse, HomoServiceStatus};
use std::{convert::Infallible, error::Error, time::Instant};

pub async fn check() -> Result<impl Reply, Infallible> {
    use warp::sse;

    let services: Vec<HomoService> = vec![];
    let (tx, rx) = futures_channel(64);
    let client = Client::builder()
        .redirect(RedirectPolicy::none())
        .build()
        .unwrap();

    for service in services {
        let subc = client.clone();
        let sender = tx.clone();
        spawn(async move {
            let response = request_service(&subc, &service).await;
            sender.send(response.map(|_| 42).map_err(|_| ()));
        });
    }

    Ok(sse::reply(rx))
}

async fn request_service(
    client: &Client,
    service: &HomoService,
) -> Result<HomoServiceResponse, Box<dyn Error + Send + Sync>> {
    let request = client.get(&service.service_url);
    let start_at = Instant::now();
    let response = request.send().await?;
    let duration = start_at.elapsed();

    // TODO セットする
    Ok(HomoServiceResponse {
        status: HomoServiceStatus::RedirectResponse,
        remote_address: response.remote_addr(),
        duration,
    })
}
