//! Contains warp filters.

use super::{action, data};
use std::{convert::Infallible, sync::Arc};

use redis::aio::Connection as RedisConnection;
use tokio::sync::Mutex;
use tokio_postgres::Client as PostgresClient;
use warp::{Filter, Rejection, Reply};

/// Holds external service connection.
#[derive(Clone)]
pub struct Connections {
    pub postgres: Arc<PostgresClient>,
    pub redis: Arc<Mutex<RedisConnection>>,
}

/// Returns the combined routes.
pub fn homochecker(
    connection: Connections,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    homochecker_check_all(connection.clone())
        .or(homochecker_check_user(connection.clone()))
        .or(homochecker_list_all(connection.clone()))
        .or(homochecker_list_user(connection.clone()))
        .or(homochecker_badge(connection))
        .with(warp::log("homochecker_rs"))
}

/// Returns a filter attaches the connection pool.
fn attach_pool(
    connection: Connections,
) -> impl Filter<Extract = (Connections,), Error = Infallible> + Clone {
    warp::any().map(move || connection.clone())
}

/// Returns the filter of `GET /check`.
fn homochecker_check_all(
    connection: Connections,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check")
        .and(warp::get())
        .and(warp::query::<data::CheckQueryParameter>())
        .and(attach_pool(connection))
        .and_then(action::check_all)
}

fn homochecker_check_user(
    connection: Connections,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check" / String)
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(connection))
        .and_then(action::check_user)
}

fn homochecker_list_all(
    connection: Connections,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("list")
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(connection))
        .and_then(action::list_all)
}

fn homochecker_list_user(
    connection: Connections,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("list" / String)
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(connection))
        .and_then(action::list_user)
}

fn homochecker_badge(
    connection: Connections,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("badge")
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(connection))
        .and_then(action::redirect_badge)
}
