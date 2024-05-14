use std::{fs::File, io::Read};

use anyhow::Ok;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Formart;

pub fn process_encode(input: &str, formart: &Base64Formart) -> anyhow::Result<()> {
    let mut reader = read_data(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let result = match formart {
        Base64Formart::STANDARD => STANDARD.encode(buf),
        Base64Formart::URL => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", result);

    Ok(())
}

pub fn process_decode(input: &str, formart: &Base64Formart) -> anyhow::Result<()> {
    let mut reader = read_data(input)?;

    let mut str = String::new();
    reader.read_to_string(&mut str)?;
    let str = str.trim();
    println!("{:?}", str);
    let result = match formart {
        Base64Formart::STANDARD => STANDARD.decode(str),
        Base64Formart::URL => URL_SAFE_NO_PAD.decode(str),
    }?;

    println!("{:?}", result);
    println!("{:?}", String::from_utf8(result));
    Ok(())
}

fn read_data(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}

#[test]
fn test_process_decode() {
    let input = "tmp/base64.txt";
    let format = Base64Formart::STANDARD;
    assert!(process_decode(input, &format).is_ok());
}
