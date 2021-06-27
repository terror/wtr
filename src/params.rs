use crate::common::*;

pub struct Params {
  city:  Option<String>,
  units: Option<String>,
  lang:  Option<String>,
}

impl Params {
  pub fn new(city: Option<String>, units: Option<String>, lang: Option<String>) -> Self {
    Self { city, units, lang }
  }

  pub fn get(&self) -> BTreeMap<&str, &Option<String>> {
    let mut params: BTreeMap<&str, &Option<String>> = BTreeMap::new();

    params.insert("q", &self.city);
    params.insert("units", &self.units);
    params.insert("lang", &self.lang);

    params
  }
}
