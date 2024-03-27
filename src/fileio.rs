use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ptr::null;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Accounts {
    name: String,
    email: String,
}

pub fn read_save_file() {

    let mut file: File = match File::open("save.json") {
        Ok(v) => v,
        Err(e) => create_save_file(),
    };

    read_accounts(file);
}

fn create_save_file() -> File {
    File::create("save.json").unwrap()
}

fn read_accounts(file: File) {

    if file.metadata().unwrap().len() == 0 { return; }
    let mut buf_reader = BufReader::new(file);

    let accounts: Accounts = serde_json::from_reader(buf_reader).unwrap();
    println!("{:#?}", accounts);
}

// fn write_accounts() {
//
//     file.write(&json_data.to_string().as_bytes());
//     serde_json::from_str(r#"{ "accounts" }"#)
// }
