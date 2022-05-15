use jsonwebtoken::{encode, errors::Error, Header, Algorithm, Validation};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    schema::users,
    services::auth::{check_hashed, hash},
    utils::constants::encoding_key,
};

use super::{token::NewToken, customer::NewCustomer};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    lastname: String,
    role_id: i32,
    password: String,
    email: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct FiltedUser {
    id: i32,
    name: String,
    lastname: String,
    role_id: i32,
    email: String,
    updated_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPayload {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPayloadLogged {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub role_id: i32,
    pub token: String
}

impl User {
    pub fn check_password(self: &User, compare: String) -> bool {
        check_hashed((*self.password).to_owned(), compare)
    }
    pub fn get_payload(self: &User) -> UserPayload {
        UserPayload {
            id: (*self).id,
            role_id: (*self).role_id,
            name: (*self.name).to_owned(),
            lastname: (*self.lastname).to_owned(),
        }
    }
    pub fn payload_with_token(self: &User, token: String)->UserPayloadLogged{
      UserPayloadLogged {
        id: (*self).id,
        name: (*self.name).to_owned(),
        role_id: (*self).role_id,
        lastname: (*self.lastname).to_owned(),
        token
    }
    }
    pub fn loging_in(self: &User) -> Result<NewToken, Error> {
        match encode(&Header::new(Algorithm::HS256), &self.get_payload(), &encoding_key()) {
            Ok(token) => Ok(NewToken::from(token, self.id)),
            Err(error) => Err(error),
        }
    }
    pub fn get_id(self: &User)->i32{
      self.id
    }
}

#[derive(Serialize, Deserialize, Debug,Validate)]
pub struct NewUser {
    #[validate(length(min = 2, max = 55))]
    name: String,
    #[validate(length(min = 2, max = 55))]
    lastname: String,
    #[validate(email)]
    email: String,
    #[validate(range(min = 1, max = 4))]
    role_id: i32,
    password: String,
    pub profile: Option<String>
}
impl NewUser {
    pub fn hash_password(self: &mut NewUser) -> &NewUser {
        match hash((*self.password).to_string()) {
            Some(hashed) => {
                self.password = hashed;
                self
            }
            _ => self,
        }
    }
    pub fn get_email(self: &NewUser) -> String {
        self.email.to_owned()
    }
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name="users"]
pub struct InsertUser{
  name: String,
  lastname: String,
  email: String,
  password: String,
}
impl InsertUser{
  pub fn from(new_user: &NewUser)-> InsertUser{
    InsertUser{
      name: new_user.name.to_owned(),
      lastname: new_user.lastname.to_owned(),
      email: new_user.email.to_owned(),
      password: new_user.password.to_owned()
    }
  }
}