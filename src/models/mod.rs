extern crate postgres;

use self::postgres::error::*;
use self::postgres::{Connection, SslMode};

pub trait Model {
    fn insert(&mut self) -> Result<&Self, Error>;
    fn update(&mut self) -> Result<&Self, DbError>;
    fn find_one() -> Result<Self, DbError>;
    fn find() -> Result<Vec<Self>, DbError>;
    fn get_db_connection() -> Result<Connection, ConnectError> {
        Connection::connect("postgres://hello_rust:hello_rust@localhost", &SslMode::None)
    }
}

pub mod user;
pub use self::user::User;
