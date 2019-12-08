use log;
use crate::content_type::ContentType;
use crate::insult::Insult;
use crate::operation::Operation;

lazy_static!(
    static ref ROUTES: Vec<Box<dyn Route>> = {
        let mut r: Vec<Box<dyn Route>> = Vec::new();
        r.push(Box::new(VersionRoute::new("2.0.0".into())));
        r.push(Box::new(OperationsRoute::new()));
        r.push(Box::new(InsultRoute::new("/anyway/:company/:from", "Who the fuck are you anyway, :company, why are you stirring up so much trouble, and, who pays you? - :from".into())));
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
            .for_each(|(field, value)| message = message.replace(&field.field, &value));
        Insult::new(message, subtitle).render(content_type)
    }
    fn matches_uri(&self, uri: &str) -> bool {
        false
    }
    fn matches_fields(&self, field_count: u32) -> bool {
        self.operation.fields.len() == field_count as usize
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
        assert!(route.resolve(ContentType::Json, &vec![]).contains("\"url\":\"/operations\""));
    }
}