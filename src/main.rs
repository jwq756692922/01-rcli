use clap::Parser;
use rcli::{
    check_pwd_safe, do_csv_process, do_gen_pwd_process, process_decode, process_encode,
    Base64SubCommand, Opts, Subcommand,
};

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
            let password = result.unwrap();
            let safe = check_pwd_safe(&password);
            println!("result:{:?} safe:{:?}", password, safe.unwrap());
        }
        Subcommand::Base64(opts) => match opts {
            Base64SubCommand::Decode(opts) => {
                println!("decode age:{:?}", opts);
                println!(
                    "decode age:{:?}",
                    process_decode(&opts.input, &opts.formart)
                );
            }
            Base64SubCommand::Encode(opts) => {
                println!("dncode age:{:?}", opts);
                println!(
                    "decode age:{:?}",
                    process_encode(&opts.input, &opts.formart)
                );
            }
        },
        _ => {
            println!("-");
        }
    }

    Ok(())
}
