extern crate hyper;
extern crate futures;
extern crate regex;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

use hyper::Server;
use hyper::service::service_fn;
use hyper::rt::Future;

mod insult_service;
mod router;
mod operation;
mod insult;
use crate::insult_service::insult;

fn main() {
    env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn(insult))
        .map_err(|e| error!("server error: {}", e));
    info!("Running at port {}", addr.port());
    hyper::rt::run(server);
}
