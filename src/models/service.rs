use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::schema::{service_types, services};

use super::customer::Customer;


#[derive(Queryable,Serialize, Deserialize)]
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
  customer_id: i32,
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
  pub type_id: i32,
  pub name: String,
  pub customer_id: i32,
  pub description: String,
  pub cost: BigDecimal,
  pub active: Option<bool>,
}

#[derive(Deserialize,Serialize,Validate)]
pub struct PostService{
  type_id: i32,
  name: String,
  #[validate(length(min= 1, max=255))]
  description: String,
  active: Option<bool>,
  cost: BigDecimal,
}
impl PostService{
  pub fn to_new_service(self: &PostService, customer: &Customer)->NewService{
    NewService{
      type_id: self.type_id,
      name: self.name.to_owned(),
      customer_id: customer.get_id(),
      description: self.description.to_owned(),
      cost: self.cost.to_owned(),
      active: self.active,
    }
  }
}
