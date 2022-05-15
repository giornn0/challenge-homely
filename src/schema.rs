table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
        valid_until -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    customers (id) {
        id -> Int4,
        user_id -> Int4,
        profile -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        active -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    service_types (id) {
        id -> Int4,
        name -> Varchar,
        active -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    services (id) {
        id -> Int4,
        name -> Varchar,
        customer_id -> Int4,
        description -> Varchar,
        cost -> Numeric,
        type_id -> Int4,
        active -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    ticket_statuses (id) {
        id -> Int4,
        name -> Varchar,
        active -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tickets (id) {
        id -> Int4,
        description -> Varchar,
        customer_id -> Int4,
        service_id -> Int4,
        in_charge_user_id -> Nullable<Int4>,
        changed_by_user_id -> Nullable<Int4>,
        status_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        lastname -> Varchar,
        role_id -> Int4,
        password -> Text,
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(customers -> users (user_id));
joinable!(services -> customers (customer_id));
joinable!(services -> service_types (type_id));
joinable!(tickets -> customers (customer_id));
joinable!(tickets -> services (service_id));
joinable!(tickets -> ticket_statuses (status_id));
joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    customers,
    roles,
    service_types,
    services,
    ticket_statuses,
    tickets,
    users,
);
