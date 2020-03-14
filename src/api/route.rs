//! Contains warp filters.

use super::{action, data};
use std::{convert::Infallible, sync::Arc};

use tokio_postgres::Client;
use warp::{Filter, Rejection, Reply};

/// Returns the combined routes.
pub fn homochecker(
    connection: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    homochecker_check_all(connection.clone())
        .or(homochecker_check_user(connection))
        .with(warp::log("homochecker_rs"))
}

/// Returns a filter attaches the connection pool.
fn attach_pool(
    connection: Arc<Client>,
) -> impl Filter<Extract = (Arc<Client>,), Error = Infallible> + Clone {
    warp::any().map(move || connection.clone())
}

/// Returns the filter of `GET /check`.
fn homochecker_check_all(
    connection: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check")
        .and(warp::get())
        .and(warp::query::<data::CheckQueryParameter>())
        .and(attach_pool(connection))
        .and_then(action::check_all)
}

fn homochecker_check_user(
    connection: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check" / String)
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(connection))
        .and_then(action::check_user)
}
