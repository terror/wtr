use crate::common::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Unable to fetch data from: {}", url))]
  RequestError { url: String, source: reqwest::Error },
}
