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

#[derive(Serialize, Deserialize)]
pub struct Ticket{
  id: i32,
  description: String,
  customer_id: i32,
  in_charge_user_id: i32,
  changed_by_user_id: i32,
  status_id: i32,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize,Serialize, AsChangeset)]
#[table_name = "tickets"]
pub struct NewTicket{
  description: String,
  customer_id: i32,
  in_charge_user_id: i32,
  changed_by_user_id: i32,
  status_id: i32,
}
