extern crate router;
use self::router::Router;

use iron::prelude::*;
use iron::status;

use controllers;

pub fn get_router() -> Router {
    let mut router = Router::new();

    router.get("/", |_: &mut Request| {
        Ok(Response::with((status::Ok, "hello world")))
    });

    router.get("/foo", controllers::foo::get_foo);
    router.get("/bar", controllers::bar::get_bar);

    router.post("/users", controllers::users::create);

    router
}
