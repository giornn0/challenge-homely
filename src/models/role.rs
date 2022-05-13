use serde::{Serialize, Deserialize};
use crate::schema::roles;

#[derive(Serialize, Deserialize)]
pub struct Role{
  id: i32,
  name: String,
  active: Option<bool>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "roles"]
pub struct NewRole{
  name: String,
  active: Option<bool>,
}
