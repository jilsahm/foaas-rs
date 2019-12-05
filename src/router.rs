use crate::content_type::ContentType;
use crate::insult::Insult;
use crate::operation::Operation;

lazy_static!(
    static ref ROUTES: Vec<Box<dyn Route>> = {
        let mut r: Vec<Box<dyn Route>> = Vec::new();
        r.push(Box::new(VersionRoute::new("2.0.0".into())));
        r.push(Box::new(OperationsRoute::new()));
        r
    };
);

trait Route: Send + Sync {
    fn get_operation(&self) -> Operation;
    fn resolve(&self, content_type: ContentType, fields: &Vec<String>) -> String;
    fn matches_uri(&self, uri: &str) -> bool;
    fn matches_fields(&self, field_count: u32) -> bool {
        field_count == 0u32
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

pub(crate) fn get_route(route: &str) -> Option<Operation> {
    ROUTES
        .iter()
        .filter(|r| r.matches_uri(route))
        .map(|o| o.get_operation())
        .next()
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
        assert_eq!(None, get_route("invalid"));
    }
    #[test]
    fn test_get_operations() {
        let route = OperationsRoute::new();
        println!("{}", route.resolve(ContentType::Json, &vec![]));
        assert!(false)
    }
}