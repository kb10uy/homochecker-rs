//! Contains application actions.

use crate::{api::*, homo::*, repository::*};
use std::{convert::Infallible, sync::Arc, time::{Duration, Instant}};

use log::{error, info};
use reqwest::{redirect::Policy as RedirectPolicy, Client, Error as ReqwestError};
use tokio::{
    spawn,
    sync::mpsc::{channel as tokio_channel},
};
use tokio_postgres::Client as PostgresClient;
use warp::{filters::sse::ServerSentEvent, http, reply, sse, Reply};

/// Performs the action of `GET /check`.
pub async fn check_all(client: Arc<PostgresClient>) -> Result<Box<dyn Reply>, Infallible> {
    let users = match User::fetch_all(client).await {
        Ok(users) => users,
        Err(e) => {
            let message = format!("Failed to fetch users: {}", e);
            error!("{}", message);
            return Ok(Box::new(reply::with_status(
                message,
                http::StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    };
    let services = users.into_iter().map(|u| HomoService::from_user(&u));

    let (tx, rx) = tokio_channel(64);
    let client = Client::builder()
        .redirect(RedirectPolicy::none())
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    // initialize 送信
    tx.clone()
        .send(Result::<_, Infallible>::Ok((
            sse::event("initialize"),
            sse::json(CheckEventInitializeData {
                count: services.len(),
            })
            .into_a(),
        )))
        .await
        .map_err(|_| "no info")
        .expect("Receiver already dropped");

    // response 送信
    for service in services {
        // TODO: エラーメッセージを送りたい
        let service = match service {
            Ok(s) => Arc::new(s),
            Err(_) => continue,
        };

        let cl = client.clone();
        let srv = service.clone();
        let sender = tx.clone();
        spawn(async move {
            let response = request_service(cl.clone(), srv.clone()).await;
            let message = match response {
                Ok(r) => (
                    sse::event("response"),
                    sse::json(CheckEventResponseData::build(&srv, &r))
                        .into_a()
                        .into_b(),
                ),
                Err(e) => (
                    sse::event("error"),
                    sse::data(e).into_b().into_b(),
                ),
            };
            sender
                .clone()
                .send(Ok(message))
                .await
                .map_err(|_| ())
                .unwrap();
        });
    }

    Ok(Box::new(sse::reply(rx)))
}

async fn request_service(
    client: Client,
    service: Arc<HomoService>,
) -> Result<HomoServiceResponse, ReqwestError> {
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
