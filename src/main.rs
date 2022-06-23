extern crate regex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::{
    env,
    fmt::{Display, Formatter},
    net::{SocketAddr, AddrParseError}, convert::Infallible,
};

use warp::{hyper::{Response, Body}, Filter, path::FullPath};

mod content_type;
mod error;
mod field;
mod insult;
mod operation;
mod rendering;
mod router;

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

fn parse_address<T>(args: T) -> Result<SocketAddr, ArgsError> 
where
    T: Iterator<Item = String>,
{
    match args.skip(1).next() {
        Some(ip) => Ok(ip.parse()?),
        None => Err(ArgsError::MissingIp),
    }
}

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
    match parse_address(env::args()) {
        Ok(addr) => {
            let api = warp::any().and(warp::path::full()).and(warp::header::<String>("Accept")).and_then(insult);
            warp::serve(api).bind(addr).await;
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