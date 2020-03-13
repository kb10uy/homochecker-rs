//! Contains application actions.

use crate::{api::*, homo::*, repository::*};
use std::{
    convert::Infallible,
    iter::repeat,
    sync::Arc,
    time::{Duration, Instant},
};

use futures::future::join_all;
use log::{error, warn};
use reqwest::{redirect::Policy as RedirectPolicy, Client, Error as ReqwestError};
use tokio::{spawn, sync::mpsc::channel as tokio_channel};
use tokio_postgres::Client as PostgresClient;
use warp::{filters::sse::ServerSentEvent, http, reply, sse, Reply};

/// Entrypoint of `GET /check`.
pub async fn check_all(
    query: CheckQueryParameter,
    client: Arc<PostgresClient>,
) -> Result<Box<dyn Reply>, Infallible> {
    let users = match UserRepository::fetch_all(client).await {
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

    check_services(users.iter(), query).await
}

/// Entrypoint of `GET /check/:user`.
pub async fn check_user(
    screen_name: String,
    query: CheckQueryParameter,
    client: Arc<PostgresClient>,
) -> Result<Box<dyn Reply>, Infallible> {
    // TODO: screen_name のバリデーション
    let users = match UserRepository::fetch_by_screen_name(client, &screen_name).await {
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

    check_services(users.iter(), query).await
}

/// Separates the `GET /check` process by query parameter.
async fn check_services(
    users: impl IntoIterator<Item = &User>,
    query: CheckQueryParameter,
) -> Result<Box<dyn Reply>, Infallible> {
    let services: Vec<_> = users
        .into_iter()
        .map(|r| match HomoService::from_user(r) {
            Ok(hs) => Some(hs),
            Err(e) => {
                warn!("Failed to construct HomoService: {}", e);
                None
            }
        })
        .flatten()
        .collect();
    let client = Client::builder()
        .redirect(RedirectPolicy::none())
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match query.format {
        Some(CheckResponseFormat::ServerSentEvent) | None => {
            check_services_sse(client, services).await
        }
        Some(CheckResponseFormat::Json) => check_services_json(client, services).await,
    }
}

/// Checks given services and make SSE response.
async fn check_services_sse(
    client: Client,
    services: Vec<HomoService>,
) -> Result<Box<dyn Reply>, Infallible> {
    let (tx, rx) = tokio_channel(64);

    // initialize 送信
    let init_message: Result<_, Infallible> = Ok((
        sse::event("initialize"),
        sse::json(CheckEventInitializeData {
            count: services.len(),
        })
        .into_a(),
    ));
    tx.clone().send(init_message).await.unwrap_or_else(|_| {
        unreachable!("Failed to send `initialize` event: Receiver already dropped");
    });

    // response 送信
    for service in services {
        let service = Arc::new(service);
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
                Err(e) => (sse::event("error"), sse::data(e).into_b().into_b()),
            };
            // rx が drop してたら何もやることはない
            match sender.clone().send(Ok(message)).await {
                Ok(()) => (),
                Err(_) => return,
            }
        });
    }

    Ok(Box::new(sse::reply(rx)))
}

/// Checks given services and make SSE response.
async fn check_services_json(
    client: Client,
    services: Vec<HomoService>,
) -> Result<Box<dyn Reply>, Infallible> {
    let clients = repeat(client);
    let result_futures = services.into_iter().zip(clients).map(|(s, c)| async {
        let service = Arc::new(s);
        match request_service(c, service.clone()).await {
            Ok(response) => Some(CheckEventResponseData::build(&service, &response)),
            Err(_) => None,
        }
    });
    let results: Vec<_> = join_all(result_futures)
        .await
        .into_iter()
        .flatten()
        .collect();

    Ok(Box::new(reply::json(&results)))
}

/// Requests to the service and validates its response whether contains appropriate link(s).
async fn request_service(
    client: Client,
    service: Arc<HomoService>,
) -> Result<HomoServiceResponse, ReqwestError> {
    use crate::validation::response::{
        ResponseHeaderValidator, ResponseHtmlValidator, ValidateResponseExt,
    };

    let request = client.get(&service.service_url);
    let start_at = Instant::now();
    let response = request.send().await?;

    let duration = start_at.elapsed();
    let remote_address = response.remote_addr();
    let status = match response.validate::<ResponseHeaderValidator>().await {
        Some(s) => s,
        None => response
            .into_validate::<ResponseHtmlValidator>()
            .await
            .unwrap_or(HomoServiceStatus::Invalid),
    };

    Ok(HomoServiceResponse {
        status,
        remote_address,
        duration,
    })
}
