use std::str::FromStr;
use regex::Regex;
use warp::{http::HeaderValue, hyper::{Request, Body}};

#[derive(Debug, PartialEq)]
pub(crate) enum ContentType {
    PlainText,
    Html,
    Json,
    Xml,
}
impl ContentType {
    pub(crate) fn to_header_value(&self) -> HeaderValue {
        match self {
            ContentType::PlainText => "text/plain".parse().unwrap(),
            ContentType::Html => "text/html".parse().unwrap(),
            ContentType::Json => "application/json".parse().unwrap(),
            ContentType::Xml => "application/xml".parse().unwrap(),
        }
    }
    pub(crate) fn from_request(req: &Request<Body>) -> Result<Self, String> {
        Ok(req.headers()
            .get_all("Accept")
            .iter()
            .map(|val| val.to_str().unwrap_or_else(|_| ""))
            .collect::<String>()
            .parse::<Self>()?
        )
    }
}
impl FromStr for ContentType {
    type Err = String;
    fn from_str(accept: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new("(html|xml|json|plain)").expect("Invalid accept pattern");
        match pattern.captures(accept) {
            Some(group) => Ok(
                match group.get(0).map(|m| m.as_str()).unwrap_or_else(|| "plain") {
                    "plain" => ContentType::PlainText,
                    "html" => ContentType::Html,
                    "json" => ContentType::Json,
                    "xml" => ContentType::Xml,
                    _ => unreachable!()
                }
            ),
            None => Err("Unsupported accept type".into()),
        }            
    }
}

#[cfg(test)]
mod tests {
    use warp::hyper::{Request, Body};

    use super::ContentType;

    #[test]
    fn test_content_type_parse_success() {
        let plain = "text/plain".parse::<ContentType>();
        assert!(plain.is_ok()); 
        assert_eq!(ContentType::PlainText, plain.unwrap());
        let plain = "text/plain2".parse::<ContentType>();
        assert!(plain.is_ok()); 
        assert_eq!(ContentType::PlainText, plain.unwrap());
    }

    #[test]
    fn test_content_type_parse_fail() {
        assert!("unknown/blub".parse::<ContentType>().is_err());
    }

    #[test]
    fn test_content_type_from_request_success() {
        let req = Request::builder().header("Accept", "text/plain;charset=UTF-8").body(Body::empty()).unwrap();
        let content_type = ContentType::from_request(&req);
        assert!(content_type.is_ok());
        assert_eq!(ContentType::PlainText, content_type.unwrap());
    }

    #[test]
    fn test_content_type_from_request_failure() {
        let req = Request::builder().body(Body::empty()).unwrap();
        let content_type = ContentType::from_request(&req);
        assert!(content_type.is_err());
    }
}