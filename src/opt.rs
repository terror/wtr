use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "wtr")]
pub enum Opt {
  #[structopt(name = "display")]
  /// Main subcommand, display weather in the terminal
  Display { city: String, lang: Option<String> },

  #[structopt(name = "json")]
  /// View current weather JSON data
  Json { city: String, lang: Option<String> },
}

impl Opt {
  pub fn run(self) -> Result<(), Error> {
    dotenv().ok();

    let client = Client::new(env::var("API_KEY").context(error::Env)?);

    match self {
      Opt::Display { city, lang } => Self::display(&city, &lang, client)?,
      Opt::Json { city, lang } => Self::json(&city, &lang, client),
    }

    Ok(())
  }

  fn display(city: &str, lang: &Option<String>, client: Client) -> Result<(), Error> {
    let mut params: BTreeMap<&str, &str> = BTreeMap::new();

    params.insert("q", city);
    if let Some(lang) = lang {
      params.insert("lang", lang);
    }

    let data: WeatherData = client.get(params)?;

    println!("{:?}", data);

    Ok(())
  }

  fn json(city: &str, lang: &Option<String>, client: Client) {}
}
