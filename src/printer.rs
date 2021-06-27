use crate::common::*;

pub struct Printer<'a> {
  data:  &'a WeatherData,
  ascii: Ascii,
}

impl<'a> Printer<'a> {
  pub fn new(data: &'a WeatherData, ascii: Ascii) -> Self {
    Self { data, ascii }
  }

  pub fn print(&self) {
    println!(
      "{} {}",
      self.ascii,
      format!(
        "{}\n{}\n{}: {}\n{}\n{:?}",
        self.data.name,
        self.data.main.temp,
        self.data.weather[0].main,
        self.data.weather[0].description,
        self.data.main.pressure,
        Utc::now()
      )
    );
  }
}
