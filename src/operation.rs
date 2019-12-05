use std::str::FromStr;
use regex::Regex;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Operation {
  pub name: String,
  pub url: String,
  pub fields: Vec<String>,
}

impl FromStr for Operation {
  type Err = String;

  fn from_str(url: &str) -> Result<Self, Self::Err> {
    let pattern = Regex::new("^/[a-zA-Z]+((/:[a-zA-Z]+)*/:from)?$").expect("Invalid pattern");
    if !pattern.is_match(url) {
      return Err(format!("{} must match the pattern {:?}", url, pattern));
    }
    let mut name = String::with_capacity(16);
    let mut fields: Vec<String> = Vec::with_capacity(2);
    url.split("/")
      .into_iter()
      .enumerate()
      .for_each(|(i, s)| {
        match i {
          0 => (),
          1 => name.push_str(s),
          _ => fields.push(s[1..].to_string()),
        }
      });
    Ok(Operation { name, url: url.to_string(), fields, })
  }
}

#[cfg(test)]
mod tests {
  use super::Operation;
  #[test]
  fn test_operation_from_str_success() {
    let o = "/pulp/:language/:from".parse::<Operation>();
    let expected = Operation { name: "pulp".to_string(), url: "/pulp/:language/:from".to_string(), fields: vec![":language".to_string(), ":from".to_string()] };
    assert!(o.is_ok());
    assert_eq!(expected, o.unwrap());
  }
  #[test]
  fn test_operation_from_str_failure() {
    let o = "invalid".parse::<Operation>();
    assert!(o.is_err());
  }
}
