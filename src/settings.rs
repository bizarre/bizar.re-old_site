use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
  pub name: String
}

impl Settings {
  pub fn new<S: Into<String>>(string: S, format: FileFormat) -> Self {
      let mut settings = Config::new();
      settings.merge(File::from_str(&string.into(), format)).unwrap();

      settings.try_into().unwrap()
  }
}