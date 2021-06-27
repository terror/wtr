use crate::common::*;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
  pub clouds:     Clouds,
  pub coord:      Coord,
  pub main:       Main,
  pub sys:        Sys,
  pub weather:    Vec<Weather>,
  pub wind:       Wind,
  pub cod:        i64,
  pub dt:         i128,
  pub id:         i128,
  pub name:       String,
  pub timezone:   i64,
  pub visibility: i64,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
  lon: f64,
  lat: f64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
  pub id:          i64,
  pub main:        String,
  pub description: String,
  pub icon:        String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
  pub temp:       f64,
  pub feels_like: f64,
  pub temp_min:   f64,
  pub temp_max:   f64,
  pub pressure:   i64,
  pub humidity:   i64,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
  speed: f64,
  deg:   i64,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
  all: i64,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
  id:      i64,
  country: String,
  sunrise: i128,
  sunset:  i128,
}
