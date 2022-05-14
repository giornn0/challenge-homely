use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use crate::schema::{service_types, services};


#[derive(Serialize, Deserialize)]
pub struct ServiceType{
  id: i32,
  name: String,
  active: Option<bool>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "service_types"]
pub struct NewServiceType{
  name: String,
  active: Option<bool>,
}


#[derive(Queryable,Serialize)]
pub struct Service{
  id: i32,
  name: String,
  description: String,
  cost: BigDecimal,
  type_id: i32,
  active: Option<bool>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "services"]
pub struct NewService{
  type_id: i32,
  name: String,
  description: String,
  cost: BigDecimal,
  active: Option<bool>,
}
