use std::{fmt, str::FromStr};

use anyhow::anyhow;
use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode string to base64")]
    Encode(Base64SubEncodeOpts),
    #[command(name = "decode", about = "Decode base64 to string")]
    Decode(Base64SubDecodeOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

#[derive(Debug, Parser)]
pub struct Base64SubEncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "Standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64SubDecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "Standard")]
    pub format: Base64Format,
}

fn parse_base64_format(value: &str) -> Result<Base64Format, anyhow::Error> {
    value.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Standard" => Ok(Base64Format::Standard),
            "UrlSafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow!("Invalid base64 format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "Standard",
            Base64Format::UrlSafe => "UrlSafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
