use std::fmt::{Formatter, Display};
use serde::Serialize;
use super::content_type::ContentType;

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
            <body>\
                <section class=\"hero is-medium is-{} is-fullheight is-bold\">\
                <div class=\"hero-body\">\
                    <div class=\"container\">\
                    <h1 class=\"title\">{}</h1>\
                    <h2 class=\"subtitle\">{}</h2>\
                    </div>\
                </div>\
                </section>\
            </body>\
        </html>\
    ")
}

pub(crate) enum BulmaColor {
    Primary,
    Dark,
}

impl Display for BulmaColor {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            BulmaColor::Primary => write!(formatter, "primary"),
            BulmaColor::Dark => write!(formatter, "dark"),
        }
    }
}

pub(crate) trait Render: Display + Serialize {
    fn color(&self) -> BulmaColor;
    fn title(&self) -> String;
    fn subtitle(&self) -> String;
    fn render(&self, content_type: ContentType) -> String {
        match content_type {
            ContentType::PlainText => format!("{}", self),
            ContentType::Html => format!(html_skeleton!(), self.color(), self.title(), self.subtitle()),
            ContentType::Json => serde_json::to_string(&self).unwrap_or("{}".into()),
            ContentType::Xml => serde_xml_rs::to_string(&self).unwrap_or("<Unknown></Unknown>".into()),
        }
    }
}
