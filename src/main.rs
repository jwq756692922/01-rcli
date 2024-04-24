use clap::Parser;
use rcli::{Opts, Subcommand};

//anyhow ? 自动处理异常
//cargo add anyhow
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let opts = Opts::parse();

    println!("opts:{:?}", opts);

    //cargo add csv
    //cargo add serde --features derive

    match opts.cmd {
        Subcommand::Csv(opts) => {
            let records = rcli::read_csv(&opts.input)?;
            println!("len:{}", records.len());
            rcli::write_json(records, &opts.output)?;
        }
        _ => {
            println!("-");
        }
    }

    Ok(())
}
