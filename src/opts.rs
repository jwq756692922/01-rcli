use std::path::Path;

use clap::Parser;
//rcli csv
//-i input.csv  输入的csv
//-o output.json 输出的json
//--header 是否带header
//-d ',' 分隔符
#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "show csv")]
    Csv(CsvOpts),
    Text,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    //value_parser 转换及检查
    #[arg(short, long, value_parser = check_input)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value = ",")]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

pub fn check_input(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file is not exist".into())
    }
}
