use std::sync::Arc;

use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

use crate::{
    models::{
        customer::Customer,
        server::{Pool, ApiResponse},
        ticket::{
            AssignTicket, ChangeTicketStatus, NewTicket, PostTicket, Ticket, TicketPageQuery,
            DetailedTicket,
        },
        user::{User, UserPayload},
    },
    services::{
        errors::{handling_db_errors, throw_error, InvalidParameter, Unauthorized},
        response::response,
    },
};

pub async fn get_tickets(
    paginator: TicketPageQuery,
    logged_user: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::{
        customers::dsl::{customers, user_id},
        tickets::{
            dsl::{created_at, customer_id, in_charge_user_id, status_id},
            table,
        },
        ticket_statuses::{self, dsl::ticket_statuses as dsl_status}
    };
    let conn = db_pool.get().unwrap();
    let mut query = table.into_boxed();
    if logged_user.role_id == 2 {
        query = query.filter(in_charge_user_id.eq(logged_user.id))
    }
    if logged_user.role_id == 4 {
        match customers
            .filter(user_id.eq(logged_user.id))
            .first::<Customer>(&conn)
        {
            Ok(customer) => query = query.filter(customer_id.eq(customer.get_id())),
            Err(error) => return handling_db_errors(error),
        }
    }

    if let Some(status) = paginator.status_id {
        query = query.filter(status_id.eq(status));
    }

    let result: Result<Vec<Ticket>, Error> = query
        .order(created_at.desc())
        .limit(paginator.take)
        .offset((paginator.page - 1) * paginator.take)
        .load::<Ticket>(&conn);
    match result{
      Ok(result_tickets) =>{  
        let status: Vec<DetailedTicket> = result_tickets.iter().map(|ticket|{
          ticket.get_detailed(dsl_status.find(ticket.get_status()).get_result(&conn).unwrap())
        }).collect();
        Ok(warp::reply::json(&ApiResponse::<Vec<DetailedTicket>>::from(&status)))
      },
      Err(error) =>handling_db_errors(error)
    }
}

pub async fn create_ticket(
    loged_user: UserPayload,
    new_ticket: PostTicket,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::{
        customers::dsl::{customers, user_id},
        tickets::dsl::tickets,
    };
    let conn = db_pool.get().unwrap();

    match customers
        .filter(user_id.eq(loged_user.id))
        .first::<Customer>(&conn)
    {
        Ok(customer) => {
            let result: Result<Ticket, Error> = diesel::insert_into(tickets)
                .values(&NewTicket::from(&customer, new_ticket))
                .get_result(&conn);
            response(result)
        }
        Err(_) => throw_error(Unauthorized::new()),
    }
}
pub async fn assign_ticket(
    ticket_id: i32,
    loged_user: UserPayload,
    data: AssignTicket,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::{
        tickets::dsl::{id, tickets},
        users::dsl::{role_id, users},
    };
    let conn = db_pool.get().unwrap();
    let user: Result<User, Error> = users
        .filter(role_id.eq(2))
        .find(data.get_in_charge())
        .get_result(&conn);
    if loged_user.role_id != 1 {
        throw_error(Unauthorized::new())
    } else if !user.is_ok() {
        throw_error(InvalidParameter::from(
            "Trying to assing to an invalid user".to_owned(),
        ))
    } else {
        let result: Result<Ticket, Error> = diesel::update(tickets.filter(id.eq(ticket_id)))
            .set(&data.set_changed_by(&loged_user))
            .get_result(&conn);
        response(result)
    }
}
pub async fn change_ticket_status(
    ticket_id: i32,
    loged_user: UserPayload,
    data: ChangeTicketStatus,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::tickets::dsl::{id, tickets};
    let conn = db_pool.get().unwrap();
    if loged_user.role_id != 4 {
        let result: Result<Ticket, Error> = diesel::update(tickets.filter(id.eq(ticket_id)))
            .set(&data.set_changed_by(&loged_user))
            .get_result(&conn);
        response(result)
    } else {
        throw_error(Unauthorized::new())
    }
}
