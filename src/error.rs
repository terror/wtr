use crate::common::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Unable to fetch data from: {}", url))]
  RequestError {
    url:    String,
    source: reqwest::Error,
  },

  #[snafu(display("Unable to load environment variable."))]
  Env { source: env::VarError },

  #[snafu(display("Unable to parse JSON data."))]
  ParseError { source: serde_json::Error },

  #[snafu(display(""))]
  BaseDirectoriesError { source: xdg::BaseDirectoriesError },

  #[snafu(display("Unable to load configuration file from: {}", path.display()))]
  LoadConfig {
    path:   PathBuf,
    source: std::io::Error,
  },
}
