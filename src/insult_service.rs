use hyper::{Body, Response, Request, Method, StatusCode};
use futures::{future, Future};
use regex::Regex;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

pub fn insult(req: Request<Body>) -> BoxFut {
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