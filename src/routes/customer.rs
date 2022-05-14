use std::sync::Arc;

use warp::{Filter, Rejection, Reply};

use crate::{
    services::{auth::with_authenticathed, server::with_pool},
    utils::constants::API, models::server::Pool, handlers::customer::get_customer,
};

pub fn customers_router(
    db_pool: &Arc<Pool>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let scope = warp::path(API).and(warp::path("customers"));
    let get_one = scope
        .and(warp::get())
        .and(warp::path::param())
        .and(with_authenticathed(&db_pool))
        .and(warp::path::end())
        .and(with_pool(db_pool.clone()))
        .and_then(get_customer);
    get_one
}
