//! Contains warp filters.

use std::{convert::Infallible, sync::Arc};

use tokio_postgres::Client;
use warp::{Filter, Rejection, Reply};

/// Returns the combined routes.
pub fn homochecker(
    connection: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    homochecker_check(connection)
}

/// Returns a filter attaches the connection pool.
fn attach_pool(
    connection: Arc<Client>,
) -> impl Filter<Extract = (Arc<Client>,), Error = Infallible> + Clone {
    warp::any().map(move || connection.clone())
}

/// Returns the filter of `GET /check`.
fn homochecker_check(
    connection: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check")
        .and(warp::get())
        .and(attach_pool(connection))
        .and_then(crate::action::check_all)
}
