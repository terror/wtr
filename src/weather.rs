use crate::common::*;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
  clouds:     Clouds,
  coord:      Coord,
  main:       Main,
  sys:        Sys,
  weather:    Vec<Weather>,
  wind:       Wind,
  cod:        i64,
  dt:         i128,
  id:         i128,
  name:       String,
  timezone:   i64,
  visibility: i64,
}

#[derive(Debug, Deserialize)]
pub struct Coord {
  lon: f64,
  lat: f64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
  id:          i64,
  main:        String,
  description: String,
  icon:        String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
  temp:       f64,
  feels_like: f64,
  temp_min:   f64,
  temp_max:   f64,
  pressure:   i64,
  humidity:   i64,
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
