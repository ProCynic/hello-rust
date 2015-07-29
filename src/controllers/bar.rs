// bar controllers

use iron::prelude::*;
use iron::status;

pub fn get_bar(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "here's a bar")))
}
