use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 2 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q2.txt").expect("Something went wrong");
        
        let input: Vec<&str> = input_str.split("\n").collect();

        println!("Part A: {}", q.part_one(&input));
        println!("Part B: {}", q.part_two(&input));
    }

    fn part_one(&self, input: &Vec<&str>) -> i32 {
        let mut x_pos = 0;
        let mut depth = 0;

        for i in 0..input.len() {
           let split_string = input[i].split(" ").collect::<Vec<&str>>();
           
           let command = split_string[0];
           let num: i32 = split_string[1].parse().unwrap();

           if command == "forward" {
               x_pos += num;
           } else if command == "up" {
               depth -= num;
           } else if command == "down" {
               depth += num;
           }
        }

        return x_pos * depth;
    }

    fn part_two(&self, input: &Vec<&str>) -> i32 {
        let mut x_pos = 0;
        let mut aim = 0;
        let mut depth = 0;

        for i in 0..input.len() {
           let split_string = input[i].split(" ").collect::<Vec<&str>>();
           
           let command = split_string[0];
           let num: i32 = split_string[1].parse().unwrap();

           if command == "forward" {
               x_pos += num;
               depth += aim * num;
           } else if command == "up" {
               aim -= num;
           } else if command == "down" {
               aim += num;
           }
        }

        return x_pos * depth;
    }
}