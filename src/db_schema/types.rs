use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable,Serialize)]
#[diesel(table_name = crate::db_schema::schema::boardgames)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BoardGame {
    id: i32,
    name: String,
}