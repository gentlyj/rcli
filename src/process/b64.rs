use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

use crate::cli::Base64Format;

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<()> {
    println!("base64 decode");
    let mut reader = get_reader(input)?;
    let mut buf = String::new();

    reader.read_to_string(&mut buf)?;

    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}
pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<()> {
    // tips
    // 我在 windows 下常用的 shell 是 git base
    // 作为标准输入输入时, 输入文本后, 输入结束的方法是:
    // 1. enter
    // 2. ctrl+z
    // 3. enter
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    print!("{}", encoded);

    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let result = process_base64_encode(input, format);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_base64_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        let result = process_base64_decode(input, format);
        print!("123{:?}", result);
        assert!(result.is_ok());
    }
}
