// foo controllers

use iron::prelude::*;
use iron::status;

pub fn get_foo(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "here's a foo")))
}
