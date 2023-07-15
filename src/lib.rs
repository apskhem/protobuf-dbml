#![forbid(unsafe_code)]
#![forbid(clippy::all)]

pub mod generator;
pub mod transpiler;

use std::{error::Error, fs};

pub const DEFAULT_SCHEMA: &'static str = "public";
pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn transpile(config: transpiler::config::Config) -> Result<(), Box<dyn Error>> {
  if let Some(err_msg) = config.validate() {
    return Err(err_msg.into());
  }

  let sem_ast = dbml_rs::parse_file(&config.in_path)?;

  let result = transpiler::transpile(sem_ast, &config).unwrap_or_else(|e| panic!("{}", e));

  fs::write(config.out_path, result.as_bytes())?;

  Ok(())
}
