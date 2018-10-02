extern crate iron;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests;

mod solver;
mod model;


use iron::prelude::*;
use iron::status;
use iron::request::Body;

use std::io::Read;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
    let _server = Iron::new(handler).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let mut s: String = String::new();
    req.body.read_to_string(&mut s);
    Ok(Response::with((status::Ok, s)))
}