extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(handler).http("localhost:3000").unwrap();
}

fn handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}