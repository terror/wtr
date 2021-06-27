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

#[derive(StructOpt, Debug)]
#[structopt(name = "wtr")]
pub enum Opt {
  #[structopt(name = "display")]
  /// Main subcommand, display weather information in the terminal
  Display {
    city:  String,
    units: Option<String>,
    lang:  Option<String>,
  },

  #[structopt(name = "json")]
  /// View current weather JSON data
  Json { city: String, lang: Option<String> },
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    dotenv().ok();

    let client = Client::new(env::var("API_KEY").context(error::Env)?);

    match self {
      Opt::Display { city, units, lang } => {
        Self::display(Params::new(Some(city), units, lang), client)?
      }
      Opt::Json { city, lang } => Self::json(&city, &lang, client),
    }

    Ok(())
  }

  fn display(params: Params, client: Client) -> Result<(), Error> {
    let data: WeatherData = client.get(params)?;
    let weather: &Weather = &data.weather[0];
    Printer::new(&data, Ascii::from(weather.main.to_owned())).print();
    Ok(())
  }

  fn json(city: &str, lang: &Option<String>, client: Client) {}
}
