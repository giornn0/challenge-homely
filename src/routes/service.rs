use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::service::{create_service, get_all_services, get_all_service_types},
    models::server::Pool,
    services::{auth::with_authenticathed, server::with_pool, request::with_validated_json},
    utils::constants::API,
};

pub fn services_router(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let scope = warp::path(API).and(warp::path("services"));
    let get_all = scope
        .and(warp::get())
        .and(with_authenticathed(&db_pool))
        .and(warp::path::end())
        .and(with_pool(db_pool.clone()))
        .and_then(get_all_services);
    let get_all_types = scope
        .and(warp::get())
        .and(with_authenticathed(&db_pool))
        .and(warp::path("types"))
        .and(warp::path::end())
        .and(with_pool(db_pool.clone()))
        .and_then(get_all_service_types);
    let create = scope
        .and(warp::post())
        .and(warp::path::end())
        .and(with_authenticathed(&db_pool))
        .and(with_validated_json())
        .and(with_pool(db_pool.clone()))
        .and_then(create_service);
    get_all.or(create).or(get_all_types)
}
