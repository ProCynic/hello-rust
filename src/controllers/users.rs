use iron::prelude::*;
use iron::status;
use super::super::models;
use super::super::models::Model;

pub fn create(_: &mut Request) -> IronResult<Response> {
    let mut test_user = models::User {id: None, email: "bob@example.com".to_string(), name: Some("Bob Bobbington".to_string())};
    match test_user.insert() {
        Ok(_) => {
            let uid: String = match test_user.id {Some(id) => id.to_string(), None => "unknown".to_string()};
            Ok(Response::with((status::Created, format!("Created new user with id {}", uid))))
        }

        Err(e) => {
            Ok(Response::with((status::InternalServerError, format!("database error: {}", e))))
        }
    }
}
