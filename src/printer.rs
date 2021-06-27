use crate::common::*;

pub struct Printer<'a> {
  data:  &'a WeatherData,
  ascii: Ascii,
}

impl<'a> Printer<'a> {
  pub fn new(data: &'a WeatherData, ascii: Ascii) -> Self {
    Self { data, ascii }
  }

  pub fn print(&self) {}
}
