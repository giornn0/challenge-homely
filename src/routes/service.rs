use std::sync::Arc;

use warp::{Filter, Reply, Rejection};

use crate::{utils::constants::API, handlers::service::get_all_services, services::{auth::with_authenticathed, server::with_pool}, models::server::Pool};


pub fn services_router(db_pool: &Arc<Pool>)-> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  let scope = warp::path(API).and(warp::path("services"));
  let get_all = scope
          .and(warp::get())
          .and(with_authenticathed(&db_pool))
          .and(warp::path::end())
          .and(with_pool(db_pool.clone()))
          .and_then(get_all_services);
  get_all
}