#![deny(clippy::pedantic)]
use lcs::lcs;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut string1 = String::new();
    let mut string2 = String::new();
    if args.len() > 2 {
        string1 = fs::read_to_string(args[1].clone()).expect("problem");
        string2 = fs::read_to_string(args[2].clone()).expect("problem");
    } else {
        println!("string 1:");
        io::stdin()
            .read_line(&mut string1)
            .expect("something went wrong");

        println!("string 2:");
        io::stdin()
            .read_line(&mut string2)
            .expect("something went wrong");
    }

    string1 = string1.trim().to_string();
    string2 = string2.trim().to_string();

    let lcs = lcs(&string1, &string2, true);

    if lcs.is_empty() {
        println!("no common subsequences");
    } else {
        println!("longest common subsequence: {lcs:?}");
    }
}