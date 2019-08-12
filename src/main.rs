extern crate hex;
extern crate rustc_serialize as serialize;

use serialize::hex::FromHex;

mod english_text;
mod set_1;
mod xor;

use std::io;

use std::fs::File;
use std::io::prelude::*;

fn main() {}

fn ciphers_from_file(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents
        .split('\n')
        .map(|x| x.from_hex().unwrap())
        .collect())
}
