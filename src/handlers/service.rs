use std::sync::Arc;

use diesel::prelude::*;
use warp::{reply::Json, Rejection};

use crate::{models::{server::Pool, service::Service, user::UserPayload}, services::response::response};

pub async fn get_all_services(_: UserPayload, db_pool: Arc<Pool>)-> Result<Json, Rejection> {
    use crate::schema::services::dsl::services;
    let conn = db_pool.get().unwrap();
    let result = services.load::<Service>(&conn);
    response(result)
}