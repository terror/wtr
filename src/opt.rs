use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "wtr")]
pub enum Opt {
  #[structopt(name = "display")]
  /// Main subcommand, display weather information in the terminal
  Display {
    #[structopt(short = "c", long = "city")]
    city: Option<String>,

    #[structopt(short = "u", long = "units")]
    units: Option<String>,

    #[structopt(short = "l", long = "lang")]
    lang: Option<String>,
  },
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    dotenv().ok();

    let client = Client::new(env::var("API_KEY").context(error::Env)?);

    match self {
      Opt::Display { city, units, lang } => {
        if !city.is_some() && !units.is_some() && !lang.is_some() {
          return Self::display_from_config(client);
        }
        Self::display(Params::new(city, units, lang), client)?
      }
    }

    Ok(())
  }

  fn display(params: Params, client: Client) -> Result<(), Error> {
    let data: WeatherData = client.get(params)?;
    let weather: &Weather = &data.weather[0];
    Printer::new(&data, Ascii::from(weather.main.to_owned())).print();
    Ok(())
  }

  fn display_from_config(client: Client) -> Result<(), Error> {
    let config = Config::load()?;
    Self::display(Params::new(config.city, config.units, config.lang), client)?;
    Ok(())
  }
}
