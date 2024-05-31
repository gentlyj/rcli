mod base64;
mod csv;
mod genpass;
use anyhow::anyhow;
use clap::Parser;
use std::path::Path;

pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenpassOpts;

use self::base64::Base64SubCommand;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about = "Rust CLI tools", author)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert csv to json")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, anyhow::Error> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow!("File not exist: {}", filename))
    }
}
