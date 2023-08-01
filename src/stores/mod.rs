use crate::db_schema::types::BoardGame;

pub mod postgres;

pub trait Store {
    fn select_boardgames(&self) -> Vec<BoardGame>;
}