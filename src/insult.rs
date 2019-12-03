use std::fmt::{Display, Formatter};
use serde::{Serialize};

#[derive(Serialize)]
pub struct Insult {
  message: String,
  subtitle: String,
}

impl Insult {
  pub fn new(message: String, subtitle: String) -> Self {
    Insult { message, subtitle, }
  }
  pub fn to_html(&self) -> String {
    format!("\
      <div>\
        <h1>{}</h1>\
        <p>{}</p>\
      </div>\
    ", self.message, self.subtitle)
  }
}

impl Display for Insult {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(formatter, "{} {}", self.message, self.subtitle)
  }
}

#[cfg(test)]
mod tests {
  use super::Insult;
  #[test]
  fn test_insult_to_html() {
    let insult = Insult::new("Rust, motherfucker, do you speak it?".to_string(), "UnitTest".to_string());
    let insult = insult.to_html();
    assert!(insult.contains("<h1>Rust"));
    assert!(insult.contains("UnitTest</p>"));
  }
  #[test]
  fn test_insult_display() {
    let insult = Insult::new("You fucktard".to_string(), "UnitTest".to_string());
    assert_eq!(format!("{}", insult), "You fucktard UnitTest".to_string());
  }
  #[test]
  fn test_insult_json() {
    let insult = Insult::new("This is fucking awesome".to_string(), "UnitTest".to_string());
    let json = serde_json::to_string(&insult).unwrap();
    assert_eq!("{\"message\":\"This is fucking awesome\",\"subtitle\":\"UnitTest\"}", json);
  }
}