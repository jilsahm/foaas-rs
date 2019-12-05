extern crate hyper;
extern crate futures;
extern crate regex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

use futures::{future, Future};
use hyper::{Body, Response, Request, Method, StatusCode, Server};
use hyper::service::service_fn;

mod content_type;
mod router;
mod operation;
mod insult;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn insult(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => {            
            info!("{:?}", req.uri().path().split("/").collect::<Vec<&str>>());
            *response.body_mut() = Body::from("Some");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    Box::new(future::ok(response))
}

fn main() {
    env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn(insult))
        .map_err(|e| error!("server error: {}", e));
    info!("Running at port {}", addr.port());
    hyper::rt::run(server);
}
