use crate::common::*;

pub struct Client {
  api_key:  String,
  base_url: String,
  client:   blocking::Client,
}

impl Client {
  pub fn new(api_key: String) -> Self {
    Self {
      api_key,
      base_url: String::from("https://api.openweathermap.org/data/2.5/weather?"),
      client: blocking::Client::new(),
    }
  }

  pub fn get(&self, params: BTreeMap<&str, &str>) -> Result<WeatherData, Error> {
    let mut request_url = String::from(&self.base_url);

    for (key, val) in &params {
      request_url.push_str(&format!("{}={}&", key, val));
    }

    request_url.push_str(&format!("appid={}", self.api_key));

    let resp = self
      .client
      .get(&request_url)
      .send()
      .context(error::RequestError { url: request_url })?;

    let weather_data: WeatherData = from_str(&resp.text().unwrap()).context(error::ParseError)?;

    Ok(weather_data)
  }
}
