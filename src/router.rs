use std::collections::HashMap;
use crate::operation::Operation;

lazy_static! {
    static ref ROUTES: HashMap<&'static str, (Operation, &'static str)> = {
        let mut r = HashMap::new();
        r.insert("version", ("/version".parse().unwrap(), "Version 2.0.0"));
        r
    };
}

pub(crate) fn get_route(route: &str) -> Option<Operation> {
    ROUTES.get(route).map(|value| value.0.clone())
}

pub(crate) fn get_operations() -> String {
    serde_json::to_string(
        &ROUTES
            .values()
            .map(|(o, _)| o.clone())
            .collect::<Vec<Operation>>()
    ).expect("Serialization error for operations vec")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_route_some() {
        assert!(get_route("version").is_some());
    }
    #[test]
    fn test_get_route_none() {
        assert_eq!(None, get_route("invalid"));
    }
    #[test]
    fn test_get_operations() {
        println!("{}", get_operations());
        assert!(false)
    }
}