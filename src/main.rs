extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    const PORT: i32 = 3000;

    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "hello world")))
    }).http("localhost:3000").unwrap();
}
