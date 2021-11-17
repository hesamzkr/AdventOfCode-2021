#![allow(warnings)]
use std::env;
mod modules;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(q_num) = args.get(1) {
        match q_num.parse::<u32>() {
            Ok(1) => modules::q1::Solution::run(),
            Err(i) => println!("Error: {}", i),
            _ => println!("Invalid question number"),
        }
    }
}
