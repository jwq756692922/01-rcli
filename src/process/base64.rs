use std::{fs::File, io::Read};

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Formart;

pub fn process_encode(input: &str, formart: &Base64Formart) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let result = match formart {
        Base64Formart::STANDARD => STANDARD.encode(&buf),
        Base64Formart::URL => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", result);

    Ok(())
}

pub fn process_decode(input: &str, formart: &Base64Formart) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;

    let result = match formart {
        Base64Formart::STANDARD => STANDARD.decode(&buf),
        Base64Formart::URL => URL_SAFE_NO_PAD.decode(&buf),
    }?;

    println!("{:?}", result);
    println!("{:?}", String::from_utf8(result));
    Ok(())
}
