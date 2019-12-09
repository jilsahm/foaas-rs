extern crate hyper;
extern crate futures;
extern crate regex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use futures::{future, Future};
use hyper::{Body, Response, Request, Method, StatusCode, Server};
use hyper::service::service_fn;

mod content_type;
mod field;
mod router;
mod operation;
mod insult;
use crate::content_type::ContentType;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn setup_logger() {
    const LOG_VAR: &'static str = "RUST_LOG";
    match env::var(LOG_VAR) {
        Ok(_) => env_logger::init(),
        Err(e) => { 
            env::set_var(LOG_VAR, "info");
            env_logger::init();
            info!("Failed to read environment variable {} because {}, set log level to info", LOG_VAR, e);
        }
    };
}

fn insult(req: Request<Body>) -> BoxFut {
    let mut res = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => router::prepare_response(&req, &mut res),
        _ => *res.status_mut() = StatusCode::METHOD_NOT_ALLOWED,
    };
    info!("Sending response with status code {}", res.status());
    Box::new(future::ok(res))
}

fn main() {
    setup_logger();
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn(insult))
        .map_err(|e| error!("server error: {}", e));
    info!("Running at port {}", addr.port());
    hyper::rt::run(server);
}
