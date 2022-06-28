use std::str::FromStr;

use reqwest::header::HeaderValue;

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    PlainText,
    Html,
    Json,
    Xml,
}

impl ContentType {
    
    pub fn to_header_value(&self) -> HeaderValue {
        match self {
            ContentType::PlainText => "text/plain".parse().unwrap(),
            ContentType::Html => "text/html".parse().unwrap(),
            ContentType::Json => "application/json".parse().unwrap(),
            ContentType::Xml => "application/xml".parse().unwrap(),
        }
    }
}

impl FromStr for ContentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plain" => Ok(ContentType::PlainText),
            "html" => Ok(ContentType::Html),
            "json" => Ok(ContentType::Json),
            "xml" => Ok(ContentType::Xml),
            _ => Err(format!("'{}' does not match any of (plain, html, json, xml)", s))            
        }            
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::ContentType;


    #[test]
    fn from_str() {
        vec![
            ("a", Err("'a' does not match any of (plain, html, json, xml)".to_string())),
            ("plain", Ok(ContentType::PlainText)),
            ("PLAIN", Ok(ContentType::PlainText)),
            ("html", Ok(ContentType::Html)),
            ("json", Ok(ContentType::Json)),
            ("xml", Ok(ContentType::Xml)),
        ]
        .into_iter()
        .map(|(input, expected)| (ContentType::from_str(input), expected))
        .for_each(|(result, expected)| assert_eq!(expected, result));
    }
}