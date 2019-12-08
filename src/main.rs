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
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => {            
            info!("{:?}", req.uri().path().split("/").collect::<Vec<&str>>());
            *response.status_mut() = router::get_route(req.uri().path())
                .map(|_route| {
                    *response.body_mut() = Body::from("Some");
                    StatusCode::OK
                }).unwrap_or_else(|| StatusCode::NOT_FOUND);            
        },
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        },
    };
    Box::new(future::ok(response))
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
