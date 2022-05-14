use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::schema::{ticket_statuses, tickets};

use super::customer::Customer;

#[derive(Serialize, Deserialize)]
pub struct TicketStatus{
  id: i32,
  name: String,
  active: Option<bool>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "ticket_statuses"]
pub struct NewTicketStatus{
  name: String,
  active: Option<bool>,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Ticket{
  id: i32,
  description: String,
  customer_id: i32,
  service_id: i32,
  in_charge_user_id: Option<i32>,
  changed_by_user_id: Option<i32>,
  status_id: i32,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset, Validate)]
#[table_name = "tickets"]
pub struct NewTicket{
  #[validate(length(min = 2, max = 255))]
  description: String,
  customer_id: i32,
  service_id: i32,
  in_charge_user_id: Option<i32>,
  changed_by_user_id: Option<i32>,
  status_id: i32,
}
impl NewTicket{
  pub fn from(customer: &Customer, ticket: PostTicket)->NewTicket{
      NewTicket{
        description: ticket.description,
        customer_id: customer.get_id(),
        service_id: ticket.service_id,
        status_id: 1,
        in_charge_user_id: None,
        changed_by_user_id: None,
      }
  }
}

#[derive(Deserialize,Serialize,Validate)]
pub struct PostTicket{
  #[validate(length(min = 2, max = 255))]
  description: String,
  service_id: i32,
}

#[derive(Deserialize,Serialize, AsChangeset, Validate)]
#[table_name = "tickets"]
pub struct AssignTicket{
  in_charge_user_id: i32,
  changed_by_user_id: i32,
  status_id: i32,
}
impl AssignTicket{
  pub fn get_in_charge(&self)-> i32{
    self.in_charge_user_id
  }
}
#[derive(Deserialize,Serialize, AsChangeset,Validate)]
#[table_name = "tickets"]
pub struct ChangeTicketStatus{
  status_id: i32,
  changed_by_user_id: i32
}

#[derive(Deserialize, Debug)]
pub struct TicketPageQuery {
    pub page: i64,
    pub take: i64,
    pub search: Option<String>,
    pub in_charge_user_id: Option<i32>,
    pub status_id: Option<i32>
}