// stdlib
pub use std::{collections::BTreeMap, env};

// dependencies
pub use dotenv::dotenv;
pub use reqwest::{self, blocking};
pub use serde::Deserialize;
pub use serde_json::{self, from_str};
pub use snafu::{ResultExt, Snafu};
pub use structopt::StructOpt;

// modules
pub(crate) use crate::error;

// structs and enums
pub use crate::{client::Client, error::Error, opt::Opt, weather::WeatherData};
