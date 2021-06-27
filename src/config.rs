use crate::common::*;

const FILENAME: &str = "wtr.toml";

const DEFAULT: &str = "\
city  = 'toronto'
units = 'standard'
lang  = 'en'
";

#[derive(Debug, Default, Deserialize)]
pub struct Config {
  pub city:  Option<String>,
  pub units: Option<String>,
  pub lang:  Option<String>,
}

impl Config {
  pub fn default() -> &'static str {
    DEFAULT
  }

  pub fn filename() -> &'static str {
    FILENAME
  }

  pub fn load() -> Result<Self, Error> {
    let path = xdg::BaseDirectories::with_prefix(dirs::home_dir().unwrap())
      .context(error::BaseDirectoriesError)?
      .find_config_file(Self::filename());

    if let Some(path) = path {
      let path = &path;
      let text = fs::read_to_string(path).context(error::LoadConfig { path })?;
      Ok(toml::from_str(&text).unwrap())
    } else {
      Ok(toml::from_str(Self::default()).unwrap())
    }
  }
}
