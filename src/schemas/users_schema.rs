use diesel::prelude::*;

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        enabled -> Bool,
        deleted -> Bool,
        created_at -> Varchar,
    }
}