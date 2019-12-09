use log;
use hyper::{Body, Request, Response, StatusCode};
use crate::content_type::ContentType;
use crate::insult::Insult;
use crate::operation::Operation;
use crate::rendering::Render;

lazy_static!(
    static ref ROUTES: Vec<Box<dyn Route>> = {
        let mut r: Vec<Box<dyn Route>> = Vec::new();
        r.push(Box::new(VersionRoute::new("2.0.0".into())));
        r.push(Box::new(OperationsRoute::new()));
        r.push(Box::new(InsultRoute::new("/anyway/:company/:from", "Who the fuck are you anyway, :company, why are you stirring up so much trouble, and, who pays you?".into())));
        r
    };
);

trait Route: Send + Sync {
    fn get_operation(&self) -> Operation;
    fn resolve(&self, content_type: ContentType, fields: &Vec<String>) -> String;
    fn matches_uri(&self, uri: &str) -> bool;
    fn matches_fields(&self, field_count: usize) -> bool {
        field_count == 0usize
    }
}

struct OperationsRoute;

impl OperationsRoute {
    fn new() -> Self {
        OperationsRoute
    }
}

impl Route for OperationsRoute {
    fn get_operation(&self) -> Operation {
        "/operations".parse().unwrap()
    }
    fn resolve(&self, _: ContentType, _: &Vec<String>) -> String {
        serde_json::to_string(
            &ROUTES
                .iter()
                .map(|o| o.get_operation())
                .collect::<Vec<Operation>>()
        ).expect("Serialization error for operations vec")
    }
    fn matches_uri(&self, uri: &str) -> bool {
        uri == "/operations"
    }
}

struct VersionRoute(String);

impl VersionRoute {
    fn new(version: String) -> Self {
        VersionRoute(version)
    }
}

impl Route for VersionRoute {
    fn get_operation(&self) -> Operation {
        "/version".parse().unwrap()
    }
    fn resolve(&self, content_type: ContentType, _: &Vec<String>) -> String {
        Insult::new(format!("Version {}", self.0), "foaas-rs".into()).render(content_type)
    }
    fn matches_uri(&self, uri: &str) -> bool {
        uri == "/version"
    }
}

struct InsultRoute {
    operation: Operation,
    template: String,
}

impl InsultRoute {
    fn new(uri: &str, template: String) -> Self {
        InsultRoute {
            operation: uri.parse().map_err(|e| error!("{}", e)).unwrap(),
            template,
        }
    }
}

impl Route for InsultRoute {
    fn get_operation(&self) -> Operation {
        self.operation.clone()
    }
    fn resolve(&self, content_type: ContentType, params: &Vec<String>) -> String {
        let mut message = self.template.clone();
        let subtitle = params.last().map(|sub| sub.clone()).unwrap_or_else(|| "".into());
        self.operation.fields
            .iter()
            .zip(params.iter())
            .for_each(|(field, value)| {
                message = message.replace(&format!(":{}", &field.field), &value);
            });
        Insult::new(message, subtitle).render(content_type)
    }
    fn matches_uri(&self, uri: &str) -> bool {
        uri.split("/")
            .collect::<Vec<&str>>()
            .iter()
            .skip(1)
            .next()
            .map(|part| *part == self.operation.name)
            .unwrap_or_else(|| false)
    }
    fn matches_fields(&self, field_count: usize) -> bool {
        self.operation.fields.len() == field_count
    }
}

fn get_route(uri: &str) -> Option<&Box<dyn Route>> {
    ROUTES
        .iter()
        .filter(|r| r.matches_uri(uri))
        .next()
}

fn get_params(uri: &str) -> Vec<String> {
    uri.split("/")
        .skip(2)
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
}

pub(crate) fn prepare_response(req: &Request<Body>, res: &mut Response<Body>) {
    match get_route(req.uri().path()) {
        Some(route) => {
            let params = get_params(req.uri().path());
            if route.matches_fields(params.len()) {
                match ContentType::from_request(&req) {
                    Ok(content_type) => {
                        res.headers_mut().append("Content-Type", content_type.to_header_value());
                        *res.body_mut() = route.resolve(content_type, &params).into();
                    },
                    Err(_) => *res.status_mut() = StatusCode::UNSUPPORTED_MEDIA_TYPE,
                }
            } else {
                *res.status_mut() = StatusCode::BAD_REQUEST;
            }
        },
        None => *res.status_mut() = StatusCode::NOT_FOUND,
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_route_some() {
        assert!(get_route("/version").is_some());
    }
    #[test]
    fn test_get_route_none() {
        assert!(get_route("invalid").is_none());
    }
    #[test]
    fn test_get_operations() {
        let route = OperationsRoute::new();
        assert!(route.resolve(ContentType::Json, &vec![]).contains("\"url\":\"/operations\""));
    }
    #[test]
    fn test_insult_route_matches_uri_success() {
        let route = InsultRoute::new("/pulp/:language/:from", ":language motherfucker, do you speak it?".into());
        assert!(route.matches_uri("/pulp"));
    }
    #[test]
    fn test_insult_route_matches_uri_failure() {
        let route = InsultRoute::new("/pulp/:language/:from", ":language motherfucker, do you speak it?".into());
        assert!(!route.matches_uri("/pulp2"));
    }
    #[test]
    fn test_get_params() {
        let params = get_params("/hello/world");
        assert_eq!(1usize, params.len());
        assert_eq!(Some(&"world".to_string()), params.get(0));
    }
}