use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 12 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q12.txt").expect("Something went wrong");

        let input: Vec<Vec<&str>> = input_str.split("\r\n").map(|s| s.split("-").collect::<Vec<&str>>()).collect();


        println!("Part A: {}", q.part_one(&input));
        // println!("Part B: {}", q.part_two());
    }

    fn part_one(&self, map: &Vec<Vec<&str>>) -> u32 {
        let mut start_connections: Vec<usize> = vec![];
        let paths = 0;
        for i in 0..map.len() {
            if map[i].contains(&"start") {
                // start connections
                start_connections.push(i);
            }
        }








        return paths;
    }

    // fn part_two(&self) -> u32 {

    // }
}