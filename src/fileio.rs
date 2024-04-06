use crate::{SESSION, Arc};
use std::fs::File;
use std::io::{BufReader, BufWriter, prelude::*};
use serde::{Deserialize, Serialize};

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
pub fn create_account(account: Account) {
    let mut sess = SESSION.lock().unwrap();
    let i = sess.selection.clone();
    let sess_accounts = Arc::get_mut(&mut sess.accounts).unwrap();


    sess_accounts.push(account);
    write_accounts(sess_accounts).unwrap();

    std::mem::drop(sess);
}
pub fn delete_account() {
    let mut sess = SESSION.lock().unwrap();
    let i = sess.selection.clone();
    let sess_accounts = Arc::get_mut(&mut sess.accounts).unwrap();

    sess_accounts.remove(i as usize);
    write_accounts(sess_accounts).unwrap();

    std::mem::drop(sess);
}
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

fn write_accounts(accounts: &Vec<Account>) -> std::io::Result<()> {
    let file = match File::create("save.json") {
        Ok(v) => v,
        Err(_) => create_save_file(),
    };
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &accounts)?;
    writer.flush()?;
    Ok(())
}
