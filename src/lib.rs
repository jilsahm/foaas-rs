#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use std::convert::Infallible;
pub use configuration::FoaasConfiguration;
use warp::{hyper::{Response, Body}, Filter, path::FullPath};

mod configuration;
mod content_type;
mod error;
mod field;
mod insult;
mod operation;
mod rendering;
mod router;

async fn insult(path: FullPath, content_type: String) -> Result<impl warp::Reply, Infallible> {
    let mut res = Response::new(Body::empty());
    router::prepare_response(path, content_type, &mut res);
    info!("Sending response with status code {}", res.status());
    Ok(res)
}

pub async fn run(configuration: FoaasConfiguration) {
    let api = warp::any()
        .and(warp::path::full())
        .and(warp::header::<String>("Accept"))
        .and_then(insult)
        .with(warp::log("foaas-rs"));

    info!("binding foaas api on {}", configuration.socket_address);
    warp::serve(api).bind(configuration.socket_address).await; 
}