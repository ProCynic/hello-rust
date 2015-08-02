// foo controllers

use iron::prelude::*;
use iron::status;
use super::super::models;
use super::super::models::Model;

pub fn get_foo(_: &mut Request) -> IronResult<Response> {
    let u: models::User = models::User::find_one().unwrap();
    // ok, looks like I can successfuly import a model and access it
    let uid: String = match u.id {Some(id) => id.to_string(), None => "unknown".to_string()};
    Ok(Response::with((status::Ok, format!("here's a foo for user {}", uid))))
}
