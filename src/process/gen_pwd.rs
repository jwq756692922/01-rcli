use rand::Rng;

use crate::GenPwdOpts;

//cargo add rand 随机数生成

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn do_gen_pwd_process(opts: GenPwdOpts) -> anyhow::Result<Option<String>> {
    let mut password: String = String::new();

    let mut chars: Vec<u8> = Vec::new();

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    if opts.lowercase {
        chars.extend_from_slice(LOWER);
    }

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
    }

    if opts.number {
        chars.extend_from_slice(NUMBER);
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..opts.length {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx] as char);
    }

    Ok(Some(password))
}
