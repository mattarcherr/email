use std::fs::File;
use std::io::prelude::*;

pub fn create_save_file() {
    let mut file = File::create("save.txt").unwrap();

    file.write_all(b"test").unwrap();
}
