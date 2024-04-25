use clap::Parser;
use rcli::{do_csv_process, do_gen_pwd_process, Opts, Subcommand};

//anyhow ? 自动处理异常
//cargo add anyhow
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let opts = Opts::parse();

    //cargo add csv
    //cargo add serde --features derive

    match opts.cmd {
        Subcommand::Csv(opts) => {
            // let records: Vec<rcli::Player> = rcli::read_csv_to_struct(&opts.input)?;
            println!("Csv opts:{:?}", opts);

            do_csv_process(opts)?;
            //yaml 依赖 cargo add serde-yaml
            //toml 依赖 cargo add toml
        }
        Subcommand::GenPwd(opts) => {
            println!("GenPwd opts:{:?}", opts);
            let result: Option<String> = do_gen_pwd_process(opts)?;

            println!("result:{:?}", result);
        }
        _ => {
            println!("-");
        }
    }

    Ok(())
}
