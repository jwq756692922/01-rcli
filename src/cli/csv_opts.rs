use core::fmt;
use std::str::FromStr;

use clap::Parser;

use crate::check_input;

#[derive(Debug, Parser, Clone, Copy)]
pub enum OutputFormart {
    Json,
    Yaml,
    Toml,
}
//rcli csv
//-i input.csv  输入的csv
//-o output.json 输出的json
//--header 是否带header
//-d ',' 分隔符
#[derive(Debug, Parser)]
pub struct CsvOpts {
    //value_parser 转换及检查
    #[arg(short, long, value_parser = check_input)]
    pub input: String,

    #[arg(short, long, default_value = "output")]
    pub output: String,

    #[arg(short, long, default_value = ",")]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(short, long, default_value = "json", value_parser = check_format)]
    pub format: OutputFormart,
}

pub fn check_format(format: &str) -> Result<OutputFormart, anyhow::Error> {
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormart::Json),
    //     "yaml" => Ok(OutputFormart::Yaml),
    //     "toml" => Ok(OutputFormart::Toml),
    //     _ => Err("format is invalid".into()),
    // }
    format.parse::<OutputFormart>()
}

impl FromStr for OutputFormart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormart::Json),
            "yaml" => Ok(OutputFormart::Yaml),
            "toml" => Ok(OutputFormart::Toml),
            _ => Err(anyhow::anyhow!("format is invalid")),
        }
    }
}

impl From<OutputFormart> for &'static str {
    fn from(value: OutputFormart) -> Self {
        match value {
            OutputFormart::Json => "json",
            OutputFormart::Yaml => "yaml",
            OutputFormart::Toml => "toml",
        }
    }
}

impl fmt::Display for OutputFormart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
