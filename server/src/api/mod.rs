use std::convert::Infallible;

use warp::{path::FullPath, hyper::{Response, Body}, Filter};

use crate::FoaasConfiguration;

mod content_type;
mod error;
mod field;
mod insult;
mod operation;
mod rendering;
mod router;

pub async fn create_server(configuration: &FoaasConfiguration) {
    let api = warp::any()
        .and(warp::path::full())
        .and(warp::header::<String>("Accept"))
        .and_then(insult)
        .with(warp::log("foaas-rs"));

    info!("binding foaas api on {}", configuration.socket_address);
    warp::serve(api).bind(configuration.socket_address).await;
}

async fn insult(path: FullPath, content_type: String) -> Result<impl warp::Reply, Infallible> {
    let mut res = Response::new(Body::empty());
    router::prepare_response(path, content_type, &mut res);
    info!("Sending response with status code {}", res.status());
    Ok(res)
}