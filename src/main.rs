#![feature(slice_split_once)]

mod parsing;
mod testing;
mod types;
mod word;

use parsing::parse_toki_pona;

use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match parse_toki_pona(&input) {
            Ok(result) => println!("{result}"),
            Err(error) => println!("{error}"),
        }
    }
}
