use std::fs;

use csv::{Reader, StringRecord};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::OutputFormart;

pub fn write_json<T>(records: Vec<T>, file_path: &str) -> anyhow::Result<()>
where
    T: Serialize,
{
    //cargo add serde_json
    let json = serde_json::to_string_pretty(&records)?;
    // println!("json:{:?}",json);
    fs::write(file_path, json)?;
    Ok(())
}

pub fn write_record_to_file(
    (header, records): (StringRecord, Vec<StringRecord>),
    file_path: &str,
    format: OutputFormart,
) -> anyhow::Result<()> {
    let mut values: Vec<Value> = Vec::with_capacity(records.len());

    for record in records {
        //header.iter() => header头的迭代器
        //record.iter() => record行的迭代器
        //zip -> 两个迭代器相加得到元组迭代器 [(header1,record1),(header2,record2),(header3,record3)]
        //collection::<Value> -> 将元组的迭代器转换为Value对象
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        values.push(json_value);
    }
    let text = match format {
        OutputFormart::Json => serde_json::to_string_pretty(&values)?,
        OutputFormart::Yaml => serde_yaml::to_string(&values)?,
        OutputFormart::Toml => toml::to_string(&values)?,
    };
    // let json = serde_json::to_string_pretty(&values)?;

    fs::write(file_path, text)?;

    Ok(())
}

pub fn read_csv_to_struct<T>(path: &str) -> anyhow::Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let mut reader = Reader::from_path(path)?;

    let mut records: Vec<T> = Vec::with_capacity(128);
    // let records = reader.deserialize().map(|record| record.unwrap()).collect::<Vec<Player>>();

    for result in reader.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn read_csv_to_record(path: &str) -> anyhow::Result<(StringRecord, Vec<StringRecord>)> {
    let mut reader = Reader::from_path(path)?;
    let mut result: Vec<StringRecord> = Vec::with_capacity(128);
    let headers: StringRecord = reader.headers()?.clone();
    // let mut records: Vec<T> = Vec::with_capacity(128);
    for record in reader.records() {
        let record: StringRecord = record?;
        result.push(record);
    }
    Ok((headers, result))
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
