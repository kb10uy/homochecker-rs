//! Contains warp filters.

use super::{action, data};
use crate::repository::Repositories;
use std::convert::Infallible;

use warp::{Filter, Rejection, Reply};

/// Returns the combined routes.
pub fn homochecker(
    repo: impl Repositories + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    homochecker_check_all(repo.clone())
        .or(homochecker_check_user(repo.clone()))
        .or(homochecker_list_all(repo.clone()))
        .or(homochecker_list_user(repo.clone()))
        .or(homochecker_badge(repo))
        .with(warp::log("homochecker_rs"))
}

/// Returns a filter attaches the repo pool.
fn attach_pool<R>(repo: R) -> impl Filter<Extract = (R,), Error = Infallible> + Clone
where
    R: Repositories + 'static,
{
    warp::any().map(move || repo.clone())
}

/// Returns the filter of `GET /check`.
fn homochecker_check_all(
    repo: impl Repositories + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check")
        .and(warp::get())
        .and(warp::query::<data::CheckQueryParameter>())
        .and(attach_pool(repo))
        .and_then(action::check_all)
}

/// Returns the filter of `GET /check/:user`.
fn homochecker_check_user(
    repo: impl Repositories + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("check" / String)
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(repo))
        .and_then(action::check_user)
}

/// Returns the filter of `GET /list`.
fn homochecker_list_all(
    repo: impl Repositories + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("list")
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(repo))
        .and_then(action::list_all)
}

/// Returns the filter of `GET /list/:user`.
fn homochecker_list_user(
    repo: impl Repositories + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("list" / String)
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(repo))
        .and_then(action::list_user)
}

/// Returns the filter of `GET /badge`.
fn homochecker_badge(
    repo: impl Repositories + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("badge")
        .and(warp::get())
        .and(warp::query())
        .and(attach_pool(repo))
        .and_then(action::redirect_badge)
}
