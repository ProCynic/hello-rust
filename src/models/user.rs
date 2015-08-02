use models::Model;
use super::postgres::error::*;

pub struct User {
    pub id: Option<u64>,
}

impl Model for User {
    fn insert(&mut self) -> Result<&User, DbError>{
        // e.g. send an insert statement to db, update self with db generated id
        let conn = Self::get_db_connection().unwrap();
        self.id = Some(1);
        Ok(self)
    }

    fn update(&mut self) -> Result<&User, DbError>{
        // e.g. send an update statement and update a field based on what db returns
        Ok(self)
    }

    fn find_one() -> Result<User, DbError> {
        // e.g. run a select statement on db and return only result
        Ok(User {id: Some(1)})
    }

    fn find() -> Result<Vec<User>, DbError> {
        Ok(vec![User{id: Some(1)}])
    }

}
