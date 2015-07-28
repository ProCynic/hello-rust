extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    const PORT: u16 = 3000;

    let server = Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "hello world")))
    });

    let server_started = server.http(("localhost", PORT));

    match server_started {
        Ok(_) => println!("Server started on port {}", PORT),
        Err(e) => println!("Server failed to start with error: {}", e),
    }
}
