use serde::{Serialize, Deserialize};
use diesel::Associations;
use validator::Validate;
use crate::schema::{ticket_statuses, tickets};

use super::{customer::Customer, user::UserPayload};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "ticket_statuses"]
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

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, Debug)]
#[belongs_to(TicketStatus,foreign_key = "status_id")]
#[table_name = "tickets"]
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
impl Ticket{
  pub fn get_status(self: &Ticket)->i32{
    self.status_id
  }
  pub fn get_detailed(self: &Ticket, status: TicketStatus)->DetailedTicket{
    DetailedTicket{
      id: self.id,
      description: self.description.to_owned(),
      customer_id: self.customer_id,
      service_id: self.service_id,
      in_charge_user_id: self.in_charge_user_id,
      changed_by_user_id: self.changed_by_user_id,
      status_id: self.status_id,
      status,
      created_at: self.created_at,
      updated_at: self.updated_at,
    }
  }
}
#[derive(Deserialize,Serialize)]
pub struct DetailedTicket{
  pub id: i32,
  pub description: String,
  pub customer_id: i32,
  pub service_id: i32,
  pub in_charge_user_id: Option<i32>,
  pub changed_by_user_id: Option<i32>,
  pub status_id: i32,
  pub status: TicketStatus,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
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
  status_id: i32,
  changed_by_user_id: i32,
}
impl AssignTicket{
  pub fn get_in_charge(&self)-> i32{
    self.in_charge_user_id
  }
  pub fn set_changed_by(mut self, user: &UserPayload)->AssignTicket{
    self.changed_by_user_id = user.id;
    self
  }
}
#[derive(Deserialize,Serialize, AsChangeset,Validate)]
#[table_name = "tickets"]
pub struct ChangeTicketStatus{
  status_id: i32,
  changed_by_user_id: i32,
}
impl ChangeTicketStatus{
  pub fn set_changed_by(mut self, user: &UserPayload)->ChangeTicketStatus{
    self.changed_by_user_id = user.id;
    self
  }
}
#[derive(Deserialize, Debug)]
pub struct TicketPageQuery {
    pub page: i64,
    pub take: i64,
    pub search: Option<String>,
    pub in_charge_user_id: Option<i32>,
    pub status_id: Option<i32>
}