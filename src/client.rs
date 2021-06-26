use crate::common::*;

pub struct Client {
  api_key: String,
  base_url: String,
  client: blocking::Client,
}

impl Client {
  pub fn new(api_key: String) -> Self {
    Self {
      api_key,
      base_url: String::from("api.openweathermap.org/data/2.5/weather?"),
      client: blocking::Client::new(),
    }
  }

  pub fn get(&self, params: BTreeMap<&str, &str>) -> Result<(), Error> {
    let mut request_url = String::from(&self.base_url);

    for (key, val) in &params {
      request_url.push_str(&format!("{}={}", key, val));
    }

    let resp = self
      .client
      .get(&request_url)
      .send()
      .context(error::RequestError { url: request_url })?;

    Ok(())
  }
}
