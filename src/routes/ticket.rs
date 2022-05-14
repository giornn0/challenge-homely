use std::sync::Arc;

use warp::{Filter, Reply, Rejection};

use crate::{services::{auth::with_authenticathed, server::with_pool}, utils::constants::API, models::server::Pool, handlers::ticket::get_tickets};

pub fn tickets_routes(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let scope = warp::path(API).and(warp::path("tickets"));
    let get_tickets = scope
        .and(warp::get())
        .and(warp::query())
        .and(warp::path::end())
        .and(with_authenticathed(&db_pool))
        .and(with_pool(db_pool.clone()))
        .and_then(get_tickets);
    get_tickets
}
