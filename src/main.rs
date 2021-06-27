use crate::common::*;

mod ascii;
mod client;
mod common;
mod config;
mod error;
mod opt;
mod printer;
mod weather;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
