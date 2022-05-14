use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::ticket::{assign_ticket, change_ticket_status, create_ticket, get_tickets},
    models::server::Pool,
    services::{auth::with_authenticathed, request::with_validated_json, server::with_pool},
    utils::constants::API,
};

pub fn tickets_router(
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
    let create_ticket = scope
        .and(warp::post())
        .and(warp::path::end())
        .and(with_authenticathed(&db_pool))
        .and(with_validated_json())
        .and(with_pool(db_pool.clone()))
        .and_then(create_ticket);
    let assign = scope
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_authenticathed(&db_pool))
        .and(with_validated_json())
        .and(with_pool(db_pool.clone()))
        .and_then(assign_ticket);
    let change_status = scope
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_authenticathed(&db_pool))
        .and(with_validated_json())
        .and(with_pool(db_pool.clone()))
        .and_then(change_ticket_status);
    get_tickets.or(create_ticket).or(assign).or(change_status)
}
