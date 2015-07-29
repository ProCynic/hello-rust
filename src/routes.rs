extern crate router;
use self::router::Router;

use iron::prelude::*;
use iron::status;

pub fn get_router() -> Router {
    let mut router = Router::new();

    router.get("/", |_: &mut Request| {
        Ok(Response::with((status::Ok, "hello world")))
    });

    router
}
