use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use super::super::models;
use super::super::models::Model;

extern crate rustc_serialize;
use self::rustc_serialize::json;

pub fn create(_: &mut Request) -> IronResult<Response> {
    let mut test_user = models::User {id: None, email: "bob@example.com".to_string(), name: Some("Bob Bobbington".to_string())};
    match test_user.insert() {
        Ok(_) => {
            Ok(Response::with(("application/json".parse::<Mime>().unwrap(), status::Created, json::encode(&test_user).unwrap())))
        }

        Err(e) => {
            Ok(Response::with((status::InternalServerError, format!("database error: {}", e))))
        }
    }
}
