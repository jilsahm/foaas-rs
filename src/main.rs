extern crate hyper;
extern crate futures;
extern crate regex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::{
    env,
    fmt::{Display, Formatter},
    net::{SocketAddr, AddrParseError},
};

use futures::{future, Future};
use hyper::{Body, Response, Request, Method, StatusCode, Server};
use hyper::service::service_fn;

mod content_type;
mod error;
mod field;
mod insult;
mod operation;
mod rendering;
mod router;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

#[derive(Debug, PartialEq)]
enum ArgsError {
    MissingIp,
    InvalidIp(AddrParseError),
}

impl From<AddrParseError> for ArgsError {
    fn from(error: AddrParseError) -> Self {
        ArgsError::InvalidIp(error)
    }
}

impl Display for ArgsError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ArgsError::MissingIp => write!(formatter, "missing address parameter"),
            ArgsError::InvalidIp(e) => write!(formatter, "the given ip was invalid because {}", e),
        }        
    }
}

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

fn parse_address<T>(args: T) -> Result<SocketAddr, ArgsError> 
where
    T: Iterator<Item = String>,
{
    match args.skip(1).next() {
        Some(ip) => Ok(ip.parse()?),
        None => Err(ArgsError::MissingIp),
    }
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
    match parse_address(env::args()) {
        Ok(addr) => {
            let server = Server::bind(&addr)
                .serve(|| service_fn(insult))
                .map_err(|e| error!("server error: {}", e));
            info!("Running at {}", addr);
            hyper::rt::run(server);
        }
        Err(e) => {
            error!("{}", e);
            info!("Usage: foaas-rs ipv4:port | ipv6:port");
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::{parse_address, ArgsError};

    #[test]
    fn test_parse_address_success_v4() {
        let args = vec!["unittest".to_string(), "127.0.0.1:8000".to_string()];
        let addr = parse_address(args.into_iter());
        assert!(addr.is_ok());
        assert_eq!(8000, addr.ok().unwrap().port());
    }

    #[test]
    fn test_parse_address_success_v6() {
        let args = vec!["unittest".to_string(), "[1A00::EFFA]:9000".to_string()];
        let addr = parse_address(args.into_iter());
        assert!(addr.is_ok());
        assert_eq!(9000, addr.ok().unwrap().port());
    }

    #[test]
    fn test_parse_address_missing() {
        let args = vec!["unittest".to_string()];
        let addr = parse_address(args.into_iter());
        assert!(addr.is_err());
        assert_eq!(ArgsError::MissingIp, addr.unwrap_err());
    }

    #[test]
    fn test_parse_addess_invalid() {
        let args = vec!["unittest".to_string(), "19000.a:3000".to_string()];
        let addr = parse_address(args.into_iter());
        assert!(addr.is_err());
        assert!(addr.unwrap_err().to_string().contains("the given ip was invalid because"));
    }
}