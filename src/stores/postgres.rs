use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::db_schema::schema::boardgames::dsl::*;
use crate::db_schema::types::BoardGame;
use super::Store;
use diesel::{
    r2d2::{Pool, ConnectionManager},
    pg::PgConnection
 };

 #[derive(Clone)]
pub struct PostgresStore {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresStore {
    pub fn new() -> PostgresStore {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool.");
        PostgresStore{pool}
    }
}

impl Store for PostgresStore {
    fn select_boardgames(&self) -> Vec<BoardGame> {
        let mut conn = self.pool.get().unwrap();
        boardgames
            .limit(5)
            .select(BoardGame::as_select())
            .load(&mut conn)
            .expect("Error loading posts")
    }
}





