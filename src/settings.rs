use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
  pub name: String,
  pub status: Vec<String>
}

impl Settings {
  pub fn new<S: Into<String>>(string: S) -> Self {
      serde_yaml::from_str(&string.into()).unwrap()
  }
}