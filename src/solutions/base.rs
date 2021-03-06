use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 0 };
        println!("Question {}", q.number);
        let input = fs::read_to_string("./inputs/q0.txt").expect("Something went wrong");
        println!("Part A: {}", q.part_one());
        println!("Part B: {}", q.part_two());
    }

    fn part_one(&self) -> &str {
        return "Answer to Part A";
    }

    fn part_two(&self) -> &str {
        return "Answer to Part B";
    }
}
