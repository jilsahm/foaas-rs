use std::fmt::{Display, Formatter};
use serde::Serialize;
use warp::hyper::StatusCode;
use crate::rendering::{BulmaColor, Render};

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
    fn color(&self) -> BulmaColor { BulmaColor::Dark }
    fn title(&self) -> String { self.code.to_string() }
    fn subtitle(&self) -> String { self.text.clone() }
}

impl Display for ErrorPage {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{} {}", self.code, self.text)
    }
}