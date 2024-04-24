use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

pub fn write_json(players: Vec<Player>, file_path: &str) -> anyhow::Result<()> {
    //cargo add serde_json
    let json = serde_json::to_string_pretty(&players)?;
    // println!("json:{:?}",json);
    fs::write(file_path, json)?;
    Ok(())
}

pub fn read_csv(path: &str) -> anyhow::Result<Vec<Player>> {
    let mut reader = Reader::from_path(path)?;

    let mut records: Vec<Player> = Vec::with_capacity(128);
    // let records = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();

    for result in reader.deserialize() {
        let record: Player = result?;
        // println!("record:{:?}",record);
        records.push(record);
    }

    Ok(records)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: i32,
}
