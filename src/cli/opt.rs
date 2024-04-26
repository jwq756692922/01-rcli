use clap::Parser;

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
}
