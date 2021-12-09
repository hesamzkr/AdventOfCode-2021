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
            Ok(4) => solutions::q4::Solution::run(),
            Ok(5) => solutions::q5::Solution::run(),
            Ok(6) => solutions::q6::Solution::run(),
            Ok(7) => solutions::q7::Solution::run(),
            Err(i) => println!("Error: {}", i),
            _ => println!("Invalid question number"),
        }
    }
}
