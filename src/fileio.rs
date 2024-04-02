use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ptr::null;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub name: String,
    pub email: String,
}

// pub fn get_account_names() -> Vec<String> {
//     let file: File = match File::open("save.json") {
//         Ok(v) => v,
//         Err(_) => create_save_file(),
//     };
//
//     let mut account_names: Vec<String> = Vec::new();
//
//    let accounts = read_accounts(file);
//    for account in accounts {
//        account_names.push(account.name);
//    }
//    account_names
// }

pub fn read_save_file() -> Vec<Account>{

    let file: File = match File::open("save.json") {
        Ok(v) => v,
        Err(_) => create_save_file(),
    };

    read_accounts(file)
}

fn create_save_file() -> File {
    File::create("save.json").unwrap()
}

fn read_accounts(file: File) -> Vec<Account>{

    if file.metadata().unwrap().len() == 0 { return Vec::new(); }
    let buf_reader = BufReader::new(file);
    let accounts: Vec<Account>; 

    accounts = serde_json::from_reader(buf_reader).unwrap();

    accounts
}

// fn write_accounts() {
//
//     file.write(&json_data.to_string().as_bytes());
//     serde_json::from_str(r#"{ "accounts" }"#)
// }
