use serde::{Serialize, Deserialize};
use crate::schema::{ticket_statuses, tickets};

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
  in_charge_user_id: i32,
  changed_by_user_id: i32,
  status_id: i32,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}
impl Ticket{
  pub fn assign(mut self,in_charge_user_id: i32)-> Ticket{
    self.in_charge_user_id = in_charge_user_id;
    self
  }
  pub fn change_status(mut self,status_id: i32)-> Ticket{
    self.status_id = status_id;
    self
  }
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "tickets"]
pub struct NewTicket{
  description: String,
  customer_id: i32,
  service_id: i32,
  in_charge_user_id: Option<i32>,
  changed_by_user_id: Option<i32>,
  status_id: i32,
}

#[derive(Deserialize,Serialize, AsChangeset)]
#[table_name = "tickets"]
pub struct AssignTicket{
  in_charge_user_id: i32,
  status_id: i32,
}
#[derive(Deserialize,Serialize, AsChangeset)]
#[table_name = "tickets"]
pub struct ChangeTicketStatus{
  status_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct TicketPageQuery {
    pub page: i64,
    pub take: i64,
    pub search: Option<String>,
    pub in_charge_user_id: Option<i32>,
    pub status_id: Option<i32>
}