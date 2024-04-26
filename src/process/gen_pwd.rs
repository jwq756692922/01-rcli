use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

use crate::GenPwdOpts;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn do_gen_pwd_process(opts: GenPwdOpts) -> anyhow::Result<Option<String>> {
    //cargo add rand 随机数依赖
    let mut password: Vec<u8> = Vec::new();

    let mut chars: Vec<u8> = Vec::new();

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    if opts.lowercase {
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
        chars.extend_from_slice(LOWER);
    }

    if opts.uppercase {
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
        chars.extend_from_slice(UPPER);
    }

    if opts.number {
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
        chars.extend_from_slice(NUMBER);
    }

    if opts.symbol {
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..(opts.length - password.len() as u8) {
        // let idx = rng.gen_range(0..chars.len());
        // password.push(chars[idx]);
        let c = chars.choose(&mut rng).expect("chars won't be empty");

        password.push(*c);
    }

    password.shuffle(&mut rng);

    Ok(Some(String::from_utf8(password)?))
}

pub fn check_pwd_safe(password: &str) -> anyhow::Result<u8> {
    //密码安全检查依赖 cargo add zxcvbn
    let estimatge = zxcvbn(password, &[])?;
    let result = estimatge.score();
    println!("score:{result} in (0,4). less than 3 should be considered too weak.");
    Ok(result)
}
