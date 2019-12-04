use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) enum ContentType {
    PlainText,
    Html,
    Json,
    Xml,
}
impl ContentType {
    pub(crate) fn to_header_value(&self) -> String {
        match self {
            ContentType::PlainText => "text/plain".into(),
            ContentType::Html => "test/html".into(),
            ContentType::Json => "application/json".into(),
            ContentType::Xml => "application/xml".into(),
        }
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

pub(crate) trait Route {
    fn resolve(&self, content_type: ContentType, fields: &Vec<String>) -> String;
}

pub(crate) struct VersionRoute;

#[cfg(test)]
mod tests {
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
}