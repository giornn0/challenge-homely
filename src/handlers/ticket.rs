use std::sync::Arc;

use diesel::{prelude::*, result::Error};
use warp::{reply::Json, Rejection};

use crate::{
    models::{
        customer::Customer,
        server::Pool,
        ticket::{AssignTicket, ChangeTicketStatus, NewTicket, Ticket, TicketPageQuery},
        user::UserPayload,
    },
    services::{
        errors::{throw_error, Unauthorized},
        response::response,
    },
};

pub async fn get_tickets(
    paginator: TicketPageQuery,
    _: UserPayload,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::tickets::dsl::{created_at, in_charge_user_id, tickets};
    let conn = db_pool.get().unwrap();
    let result: Result<Vec<Ticket>, Error> = tickets
        .order(created_at.desc())
        .limit(paginator.take)
        .offset((paginator.page - 1) * paginator.take)
        .load::<Ticket>(&conn);
    response(result)
}

pub async fn create_ticket(
    loged_user: UserPayload,
    new_ticket: NewTicket,
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
                .values(new_ticket)
                .get_result(&conn);
            response(result)
        }
        Err(_) => throw_error(Unauthorized::new()),
    }
}
pub async fn assignTicket(
    ticket_id: i32,
    loged_user: UserPayload,
    data: AssignTicket,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::tickets::dsl::{id, tickets};
    let conn = db_pool.get().unwrap();

    if loged_user.role_id == 1 {
        let result: Result<Ticket, Error> = diesel::update(tickets.filter(id.eq(ticket_id)))
            .set(&data)
            .get_result(&conn);
        response(result)
    } else {
        throw_error(Unauthorized::new())
    }
}
pub async fn changeTicketStatus(
    ticket_id: i32,
    _loged_user: UserPayload,
    data: ChangeTicketStatus,
    db_pool: Arc<Pool>,
) -> Result<Json, Rejection> {
    use crate::schema::tickets::dsl::{id, tickets};
    let conn = db_pool.get().unwrap();

    let result: Result<Ticket, Error> = diesel::update(tickets.filter(id.eq(ticket_id)))
        .set(&data)
        .get_result(&conn);
    response(result)
}
