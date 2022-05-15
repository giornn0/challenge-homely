use std::sync::Arc;

use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

use crate::models::service::ServiceType;
use crate::schema::services::dsl::{active, services};
use crate::{
    models::{
        server::Pool,
        service::{PostService, Service},
        user::UserPayload,
    },
    services::{errors::handling_db_errors, response::response},
};

pub async fn get_all_services(
    loged_user: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::services::{table,dsl::customer_id};
    let conn = db_pool.get().unwrap();
    let mut query = table.into_boxed().filter(active.eq(true));
    if loged_user.role_id == 4{
      query = query.filter(customer_id.eq(loged_user.id));
    }
    let result = query.load::<Service>(&conn);
    response(result)
}

pub async fn create_service(
    loged_user: UserPayload,
    post_service: PostService,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::customers::dsl::{customers, user_id};
    let conn = db_pool.get().unwrap();
    match customers.filter(user_id.eq(loged_user.id)).first(&conn) {
        Ok(customer) => {
            let result:Result<Service, Error>= diesel::insert_into(services)
                .values(&post_service.to_new_service(&customer))
                .get_result(&conn);
            response(result)
        }
        Err(error) => handling_db_errors(error),
    }
}

pub async fn get_all_service_types(_: UserPayload, db_pool: Arc<Pool>)->Result<Json, Rejection>{
  use crate::schema::service_types::dsl::service_types;
  let conn = db_pool.get().unwrap();
  let result: Result<Vec<ServiceType>,Error> = service_types.load::<ServiceType>(&conn);
  response(result)
}
