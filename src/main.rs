use clap::Parser;
use rcli::{read_csv_to_record, write_record_to_file, Opts, Subcommand};

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
            // let records: Vec<rcli::Player> = rcli::read_csv_to_struct(&opts.input)?;
            // println!("len:{}", records.len());
            // rcli::write_json(records, &opts.output)?;
            let target_path = format!("{}.{}", &opts.output, &opts.format);
            write_record_to_file(
                read_csv_to_record(&opts.input)?,
                target_path.as_str(),
                opts.format,
            )?;

            //yaml 依赖 cargo add serde-yaml
            //toml 依赖 cargo add toml
        }
        _ => {
            println!("-");
        }
    }

    Ok(())
}
