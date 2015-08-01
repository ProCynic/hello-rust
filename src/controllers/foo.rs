// foo controllers

use iron::prelude::*;
use iron::status;
use super::super::models;
use super::super::models::Model;

pub fn get_foo(_: &mut Request) -> IronResult<Response> {
    let u: models::User = models::User::find_one().unwrap();
    Ok(Response::with((status::Ok, format!("here's a foo for user {}", u.id))))
}
