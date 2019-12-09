use std::fmt::Display;
use serde::Serialize;
use crate::content_type::ContentType;

macro_rules! html_skeleton {
    () => ("\
        <DOCTYPE! html>\
        <html>\
            <head>\
                <meta charset=\"utf-8\">\
                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\
                <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bulma@0.8.0/css/bulma.min.css\">\
                <title>FOAAS-RS</title>\
            </head>\
            <body>{}</body>\
        </html>\
    ")
}

pub(crate) trait Render: Display + Serialize {
    fn to_html(&self) -> String;
    fn render(&self, content_type: ContentType) -> String {
        match content_type {
            ContentType::PlainText => format!("{}", self),
            ContentType::Html => format!(html_skeleton!(), self.to_html()),
            ContentType::Json => serde_json::to_string(&self).unwrap_or("{}".into()),
            ContentType::Xml => serde_xml_rs::to_string(&self).unwrap_or("<Unknown></Unknown>".into()),
        }
    }
}
