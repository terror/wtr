use crate::common::*;

mod client;
mod common;
mod config;
mod error;
mod opt;

fn main() {
  Opt::from_args().run();
}
