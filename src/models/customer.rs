use serde::{Serialize, Deserialize};
use crate::schema::customers;

#[derive(Serialize, Deserialize)]
pub struct Customer{
  id: i32,
  user_id: i32,
  profile: String,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "customers"]
pub struct NewCustomer{
  user_id: i32,
  profile: String,
}
