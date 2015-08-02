use models::Model;
use super::postgres::error::*;

pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub name: Option<String>
}

impl Model for User {
    fn insert(&mut self) -> Result<&User, Error>{
        // e.g. send an insert statement to db, update self with db generated id
        let conn = Self::get_db_connection().unwrap();
        let statement = conn.prepare("INSERT INTO app_user (email, name) VALUES ($1, $2) RETURNING *").unwrap();
        let result = statement.query(&[&self.email, &self.name]);
        match result {
            Ok(rows) => {
                let row = rows.get(0);
                self.id = row.get("id");
                Ok(self)
            }
            Err(e) => Err(e)
        }
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
