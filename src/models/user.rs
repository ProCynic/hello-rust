use models::Model;
use super::postgres::error::*;

pub struct User {
    pub id: Option<u64>,
    pub email: String,
    pub name: Option<String>
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
        Ok(User {id: Some(1), email: "bob@example.com".to_string(), name: None})
    }

    fn find() -> Result<Vec<User>, DbError> {
        Ok(vec![User{id: Some(1), email: "bob@example.com".to_string(), name: Some("Bob Bobbington".to_string())}])
    }

}
