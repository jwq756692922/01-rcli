use std::str::FromStr;

use clap::{arg, Parser};

use crate::check_input;

use core::fmt;
// cargo run  -- base64 encode -i "filename.file"

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "decode base64")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = check_input,default_value="-")]
    pub input: String,

    #[arg(long,default_value="STANDARD",value_parser= parser_base64_formart)]
    pub formart: Base64Formart,
}
#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = check_input,default_value="-")]
    pub input: String,

    #[arg(long,default_value="STANDARD",value_parser= parser_base64_formart)]
    pub formart: Base64Formart,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum Base64Formart {
    STANDARD,
    URL,
}

fn parser_base64_formart(formart: &str) -> Result<Base64Formart, anyhow::Error> {
    formart.parse()
}

impl FromStr for Base64Formart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "STANDARD" => Ok(Base64Formart::STANDARD),
            "URL" => Ok(Base64Formart::URL),
            _ => Err(anyhow::anyhow!("format is invalid")),
        }
    }
}

impl From<Base64Formart> for &'static str {
    fn from(value: Base64Formart) -> Self {
        match value {
            Base64Formart::STANDARD => "STANDARD",
            Base64Formart::URL => "URL",
        }
    }
}

impl fmt::Display for Base64Formart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
