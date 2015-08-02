use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use super::super::models;
use super::super::models::Model;

use bodyparser;

extern crate rustc_serialize;
use self::rustc_serialize::json;

pub fn create(req: &mut Request) -> IronResult<Response> {
    let new_user = req.get::<bodyparser::Struct<models::User>>();
    match new_user {
        Ok(Some(mut u)) => {
            match u.insert() {
                Ok(created_user) => {
                    Ok(Response::with(("application/json".parse::<Mime>().unwrap(), status::Created, json::encode(&created_user).unwrap())))
                }

                Err(e) => {
                    Ok(Response::with((status::InternalServerError, format!("database error: {}", e))))
                }
            }
        }
        Ok(None) => Ok(Response::with((status::BadRequest, "Error: body required"))),
        Err(e) => Ok(Response::with((status::InternalServerError, format!("Error, failed to parse request body with error: {}", e))))
    }
}
