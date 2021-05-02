use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
  pub name: String,
  pub status: Option<Vec<String>>,
  pub bio: String,
  pub journal_subtitle: String,
  pub sketches_subtitle: String
}

impl Settings {
  pub fn new<S: Into<String>>(string: S) -> Self {
      serde_yaml::from_str(&string.into()).unwrap()
  }
}