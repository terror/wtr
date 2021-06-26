use crate::common::*;

mod client;
mod common;
mod config;
mod error;
mod opt;
mod weather;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
