use std::fmt::{Display, Formatter};
use hyper::StatusCode;
use serde::Serialize;
use crate::rendering::Render;

#[derive(Serialize)]
pub(crate) struct ErrorPage {
    code: u16,
    text: String,
}

impl ErrorPage {
    pub(crate) fn new<T: Display>(code: StatusCode, error: &T) -> Self {
        ErrorPage {
            code: code.as_u16(),
            text: format!("{}", error),
        }
    }
}

impl Render for ErrorPage {
    fn to_html(&self) -> String {
        format!("\
            <div>\
            <h1>{}</h1>\
            <p>{}</p>\
            </div>\
        ", self.code, self.text)
    }
}

impl Display for ErrorPage {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{} {}", self.code, self.text)
    }
}