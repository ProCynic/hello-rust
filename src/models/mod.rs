extern crate postgres;

use self::postgres::error::*;

trait Model {
    fn insert(&mut self) -> Result<&Self, DbError>;
    fn update(&mut self) -> Result<&Self, DbError>;
    fn find_one() -> Result<Self, DbError>;
}

pub struct User {
    id: u64,
}

impl Model for User {
    fn insert(&mut self) -> Result<&User, DbError>{
        // e.g. send an insert statement to db, update self with db generated id
        self.id = 1;
        Ok(self)
    }

    fn update(&mut self) -> Result<&User, DbError>{
        // e.g. send an update statement and update a field based on what db returns
        Ok(self)
    }

    fn find_one() -> Result<User, DbError> {
        // e.g. run a select statement on db and return only result
        Ok(User {id: 1})
    }
}
