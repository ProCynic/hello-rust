extern crate postgres;

use self::postgres::error::*;

pub trait Model {
    fn insert(&mut self) -> Result<&Self, DbError>;
    fn update(&mut self) -> Result<&Self, DbError>;
    fn find_one() -> Result<Self, DbError>;
    fn find() -> Result<Vec<Self>, DbError>;
}

pub mod user;
