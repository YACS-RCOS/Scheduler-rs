extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate futures;

#[cfg(test)]
mod tests;

mod solver;

use hyper::{Body, Response, Server, Request};
use hyper::rt::Future;
use hyper::service::service_fn;

#[cfg(test)]
const ADDRESS: ([u8; 4], u16) = ([127,0,0,1], 3000);
#[cfg(not(test))]
const ADDRESS: ([u8; 4], u16) = ([127,0,0,1], 3000);

fn main() {
    let server = Server::bind(&ADDRESS.into())
        .serve(|| service_fn(service))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn service(req: Request<Body>) -> BoxFut {
    let mut resp = Response::new(Body::empty());
    *resp.body_mut() = req.into_body();
    return Box::new(futures::future::ok(resp));
}