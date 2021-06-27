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
  fn default() -> &'static str {
    DEFAULT
  }

  fn filename() -> &'static str {
    FILENAME
  }

  fn path() -> Result<Option<PathBuf>, Error> {
    Ok(
      xdg::BaseDirectories::with_prefix(dirs::home_dir().unwrap())
        .context(error::BaseDirectoriesError)?
        .find_config_file(Self::filename()),
    )
  }

  pub fn load() -> Result<Self, Error> {
    if let Some(path) = Self::path()? {
      let path = &path;
      let text = fs::read_to_string(path).context(error::LoadConfig { path })?;
      Ok(toml::from_str(&text).unwrap())
    } else {
      Ok(toml::from_str(Self::default()).unwrap())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_config() {
    let config = Config::load();

    assert!(config.is_ok());

    if let Some(_) = Config::path().unwrap() {
    } else {
      let config = config.unwrap();
      assert_eq!(config.city.unwrap(), String::from("toronto"));
      assert_eq!(config.units.unwrap(), String::from("standard"));
      assert_eq!(config.lang.unwrap(), String::from("en"));
    }
  }
}
