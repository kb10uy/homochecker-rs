use futures::{channel::mpsc::channel as futures_channel, prelude::*};
use reqwest::{redirect::Policy as RedirectPolicy, Client, Error as ReqwestError};
use tokio::spawn;
use warp::Reply;

use crate::{
    api,
    homo::{HomoService, HomoServiceResponse, HomoServiceStatus, Provider},
};
use std::{convert::Infallible, time::Instant};

pub async fn check() -> Result<impl Reply, Infallible> {
    use warp::{filters::sse::ServerSentEvent, sse};
    // TODO: DB から取ってくる
    let services = vec![HomoService {
        provider: Provider::Mastodon {
            screen_name: "kb10uy".into(),
            domain: "mstdn.maud.io".into(),
        },
        avatar_url: "".into(),
        service_url: "https://homo.kb10uy.org".into(),
    }];

    let (tx, rx) = futures_channel(64);
    let client = Client::builder()
        .redirect(RedirectPolicy::none())
        .build()
        .unwrap();

    // initialize 送信
    tx.clone()
        .send(Ok((
            sse::event("initialize"),
            sse::json(api::CheckEventInitializeData {
                count: services.len(),
            })
            .into_a(),
        )))
        .await
        .expect("Receiver already dropped");

    // response 送信
    for service in services {
        let subc = client.clone();
        let mut sender = tx.clone();
        spawn(async move {
            let response = request_service(&subc, &service).await;
            let data = response.map(|r| {
                (
                    sse::event("response"),
                    sse::json(api::CheckEventResponseData::build(&service, &r)).into_b(),
                )
            });
            sender.send(data).await.expect("Receiver already dropped");
        });
    }

    Ok(sse::reply(rx))
}

async fn request_service(
    client: &Client,
    service: &HomoService,
) -> Result<HomoServiceResponse, ReqwestError> {
    let request = client.get(&service.service_url);
    let start_at = Instant::now();
    let response = request.send().await?;
    let duration = start_at.elapsed();

    println!("sent");

    // TODO セットする
    Ok(HomoServiceResponse {
        status: HomoServiceStatus::RedirectResponse,
        remote_address: response.remote_addr(),
        duration,
    })
}
