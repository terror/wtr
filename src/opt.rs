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
  pub fn run(self) {
    match self {
      Opt::Display { city, lang } => Self::display(&city, &lang),
      Opt::Json { city, lang } => Self::json(&city, &lang),
    }
  }

  fn display(city: &str, lang: &Option<String>) {}

  fn json(city: &str, lang: &Option<String>) {}
}
