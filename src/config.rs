use crate::common::*;

#[derive(Debug, Default)]
pub struct Config {
  city: String,
  lang: String,
  state_code: String,
  country_code: String,
  zip_code: String,
}

impl Config {
  fn new() -> Self {
    Self::default()
  }
}
