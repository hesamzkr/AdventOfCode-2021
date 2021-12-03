#![allow(warnings)]
use std::env;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(q_num) = args.get(1) {
        match q_num.parse::<u32>() {
            Ok(1) => solutions::q1::Solution::run(),
            Ok(2) => solutions::q2::Solution::run(),
            Ok(3) => solutions::q3::Solution::run(),
            Err(i) => println!("Error: {}", i),
            _ => println!("Invalid question number"),
        }
    }
}
