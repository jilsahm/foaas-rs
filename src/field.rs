use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Field {
    pub name: String,
    pub field: String,
}