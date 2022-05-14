use std::sync::Arc;

use crate::{
    models::{
        customer::{Customer, NewCustomer},
        server::Pool,
        user::{User, UserPayload},
    },
    services::{
        errors::{throw_error, Unauthorized, QueryError},
        response::response,
    }, handlers::auth::save_token,
};
use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

pub async fn get_customer(
    id: i32,
    log_customer: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::customers::dsl::customers;
    let conn = db_pool.get().unwrap();
    if id == log_customer.id {
        let result: Result<Customer, Error> = customers.find(id).get_result(&conn);
        response(result)
    } else {
        throw_error(Unauthorized::new())
    }
}

pub fn create_customer(user: User, profile: String ,db_pool: Arc<Pool>) -> Result<Json, Rejection> {
    use crate::schema::customers::dsl::customers;
    let conn = db_pool.get().unwrap();
    let new_customer = NewCustomer::from(&user, profile);
    let result: Result<Customer, Error> = diesel::insert_into(customers)
        .values(&new_customer)
        .get_result(&conn);
    if let Ok(_) = result {
        save_token(user, &conn)
    }else{
      throw_error(QueryError::new())
    }
}
