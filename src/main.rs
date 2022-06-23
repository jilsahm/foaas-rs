extern crate regex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::{
    env,    
    convert::Infallible,
};

use clap::Parser;
use configuration::FoaasConfiguration;
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = FoaasConfiguration::parse();

    let api = warp::any()
        .and(warp::path::full())
        .and(warp::header::<String>("Accept"))
        .and_then(insult)
        .with(warp::log("foaas-rs"));

    info!("binding foaas api on {}", config.socket_address);
    warp::serve(api).bind(config.socket_address).await; 
}

