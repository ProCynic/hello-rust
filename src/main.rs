extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;

mod routes;
mod controllers;
mod models;

fn main() {
    const PORT: u16 = 3000;

    let server = Iron::new(routes::get_router());

    let server_started = server.http(("localhost", PORT));

    match server_started {
        Ok(_) => println!("Server started on port {}", PORT),
        Err(e) => println!("Server failed to start with error: {}", e),
    }
}
