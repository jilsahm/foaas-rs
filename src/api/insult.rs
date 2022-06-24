use std::fmt::{Display, Formatter};
use serde::{Serialize};
use super::rendering::{BulmaColor, Render};

#[derive(Serialize)]
pub(crate) struct Insult {
  message: String,
  subtitle: String,
}

impl Insult {
  pub fn new(message: String, subtitle: String) -> Self {
    Insult { message, subtitle, }
  }
}

impl Render for Insult {
  fn color(&self) -> BulmaColor { BulmaColor::Primary }
  fn title(&self) -> String { self.message.clone() }
  fn subtitle(&self) -> String { self.subtitle.clone() }
}

impl Display for Insult {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
    write!(formatter, "{} {}", self.message, self.subtitle)
  }
}

#[cfg(test)]
mod tests {
  use crate::api::content_type::ContentType;
  use super::{Insult, Render};

  #[test]
  fn test_insult_to_html() {
    let insult = Insult::new("Rust, motherfucker, do you speak it?".to_string(), "UnitTest".to_string());
    let insult = insult.render(ContentType::Html);
    assert!(insult.contains("<h1 class=\"title\">Rust"));
    assert!(insult.contains("<h2 class=\"subtitle\">UnitTest</h2>"));
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
