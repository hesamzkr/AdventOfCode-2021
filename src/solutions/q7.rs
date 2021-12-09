use std::fs;
use std::collections::HashMap;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 7 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q7.txt").expect("Something went wrong");

        let input: Vec<i64> = input_str.split(",").map(|s| s.parse().unwrap()).collect();


        println!("Part A: {}", q.part_one(&input));
        println!("Part B: {}", q.part_two(&input));
    }

    fn part_one(&self, input: &Vec<i64>) -> i32 {
        let mut lowest_fuel = 1_000_000;
        for pos in 1..2000 {
            let mut fuel = 0;
            for i in input {
                fuel += i32::abs(*i as i32 - pos);
            }

            if fuel < lowest_fuel {
                lowest_fuel = fuel;
            }
        }

        return lowest_fuel;
    }

    fn part_two(&self, input: &Vec<i64>) -> i64 {
        let mut lowest_fuel: i64 = 1_000_000_000;
        for pos in 1..2000 {
            let mut fuel: i64 = 0;
            for i in input {
                let n = i64::abs(i - pos) as f64;
                let s = (n * 0.5) * (n + 1.0);
                fuel += s as i64;
            }

            if fuel < lowest_fuel {
                lowest_fuel = fuel;
            }
        }

        return lowest_fuel;
    }
}
