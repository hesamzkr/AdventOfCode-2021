use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 1 };
        println!("Question {}", q.number);

        let input_str = fs::read_to_string("./inputs/q1.txt").expect("Something went wrong");
        
        let input: Vec<i32> = input_str.split("\n").map(|s| s.parse().expect("Can't parse to i32")).collect();

        println!("Part A: {}", q.part_one(&input));
        println!("Part B: {}", q.part_two(&input));
    }

    fn part_one(&self, input: &Vec<i32>) -> i32 {
        let mut previous = input[0];
        let mut increased = 0;
        for i in 1..input.len() {
            if input[i] > previous {
                increased += 1;
            }
            previous = input[i];
        }

        return increased;
    }

    fn part_two(&self, input: &Vec<i32>) -> i32 {
        let mut previous = input[0] + input[1] + input[2];
        let mut increased = 0;
        for i in 1..input.len() - 2 {
            let new = input[i] + input[i + 1] + input[i + 2];
            if new > previous {
                increased += 1;
            }

            previous = new;

        }

        return increased;
    }
}