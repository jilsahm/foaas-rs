use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Field {
    pub name: String,
    pub field: String,
}

impl Field {
    pub(crate) fn new(field: &str) -> Result<Self, String> {
        if field.len() == 0 {
            return Err("Empty fields are not permitted".into());
        }
        Ok(Field {
            name: Field::transform_name(field),
            field: field.into(),
        })
    }
    fn transform_name(field: &str) -> String {
        format!("{}{}", field.get(0..1).unwrap().to_uppercase(), field.get(1..).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Field;
    #[test]
    fn test_field_transform_name() {
        let result = Field::transform_name("hello");
        assert_eq!("Hello".to_string(), result);
    }
    #[test]
    fn test_field_new_success() {
        let f = Field::new("blub");
        assert!(f.is_ok());
        assert_eq!(Field { name: "Blub".into(), field: "blub".into(), }, f.unwrap());
    }
    #[test]
    fn test_field_new_failure() {
        let f = Field::new("");
        assert!(f.is_err());
    }
}