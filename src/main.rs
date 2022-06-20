use crate::common::*;

mod arguments;
mod checker;
mod common;
mod config;
mod guesser;
mod language;
mod parser;
mod path_ext;
mod query_config;
mod rule;
mod walker;

fn main() {
  env::remove_var("TEST");
  if let Err(error) = Arguments::parse().run() {
    eprintln!("{}: {error}", "error".red());
    process::exit(1);
  }
}
