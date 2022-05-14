use serde::{Serialize, Deserialize};
use crate::schema::customers;

use super::user::{User, UserPayload};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Customer{
  id: i32,
  user_id: i32,
  profile: String,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}
impl Customer{
  pub fn get_details(self: &Customer, matched_user: &UserPayload)->CustomerDetails{
      CustomerDetails::from(matched_user, self.profile.to_owned())
  }
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "customers"]
pub struct NewCustomer{
  user_id: i32,
  profile: String,
}
impl NewCustomer {
  pub fn from(user: &User, profile: String)-> NewCustomer{
    NewCustomer{
        user_id: user.get_id(),
        profile
    }
  }
}
#[derive(Deserialize,Serialize)]
pub struct CustomerDetails{
  name: String,
  lastname: String,
  profile: String,
}
impl CustomerDetails{
  pub fn from(user: &UserPayload, profile: String)->CustomerDetails{
    CustomerDetails{
      name: user.name.to_owned(),
      lastname: user.lastname.to_owned(),
      profile
    }
  }
}