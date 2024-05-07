use std::path::Path;

use clap::Parser;

use crate::Base64SubCommand;
use crate::CsvOpts;
use crate::GenPwdOpts;

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

    #[command(name = "genpwd", about = "generate a random password")]
    GenPwd(GenPwdOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn check_input(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file is not exist".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::check_input;

    #[test]
    fn test_check_input() {
        assert_eq!(check_input("-"), Ok("-".into()));
        assert_eq!(check_input("*"), Err("file is not exist".into()));
        assert_eq!(
            check_input("assets/juventus.csv"),
            Ok("assets/juventus.csv".into())
        );
    }
}
