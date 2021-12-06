use std::fs;
use std::collections::HashMap;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 5 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q5.txt").expect("Something went wrong");

        let input: Vec<&str> = input_str.split("\r\n").collect();


        println!("Part A: {}", q.part_one(&input));
        println!("Part B: {}", q.part_two(&input));
    }

    fn part_one(&self, input: &Vec<&str>) -> i32 {
        let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
        for i in 0..1000 {
            for j in 0..1000 {
                grid.insert((i, j), 0);
            }
        }

        for line_segment in input {
            let cords: Vec<&str> = line_segment.split("->").collect();
            let from_cords: Vec<i32> = cords[0].split(",").map(|s| s.trim().parse().expect("Can't parse to i32")).collect();
            let to_cords: Vec<i32> = cords[1].split(",").map(|s| s.trim().parse().expect("Can't parse to i32")).collect();

            let mut coef = 1;
            if to_cords[1] - from_cords[1] < 0 || to_cords[0] - from_cords[0] < 0 {
                coef = -1;
            }

            if from_cords[0] == to_cords[0] {
                for x in 0..i32::abs(to_cords[1] - from_cords[1]) + 1 {
                    *grid.get_mut(&(from_cords[0], from_cords[1] + x * coef)).unwrap() += 1;
                }
            } 
            else if from_cords[1] == to_cords[1] {
                for x in 0..i32::abs(to_cords[0] - from_cords[0]) + 1 {
                    *grid.get_mut(&(from_cords[0] + x * coef, from_cords[1])).unwrap() += 1;
                }
            }
        }

        let mut count = 0;
        for (_, v) in grid.iter() {
            if *v >= 2 {
                count += 1;
            }
        }

        return count;
    }

    fn part_two(&self, input: &Vec<&str>) -> i32 {
        let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
        for i in 0..1000 {
            for j in 0..1000 {
                grid.insert((i, j), 0);
            }
        }

        for line_segment in input {
            let cords: Vec<&str> = line_segment.split("->").collect();
            let from_cords: Vec<i32> = cords[0].split(",").map(|s| s.trim().parse().expect("Can't parse to i32")).collect();
            let to_cords: Vec<i32> = cords[1].split(",").map(|s| s.trim().parse().expect("Can't parse to i32")).collect();

            let mut x_coef = 1;
            let mut y_coef = 1;
            if to_cords[0] - from_cords[0] < 0 {
                x_coef = -1;
            } else if to_cords[0] - from_cords[0] == 0 {
                x_coef = 0;
            }

            if to_cords[1] - from_cords[1] < 0 {
                y_coef = -1;
            } else if to_cords[1] - from_cords[1] == 0 {
                y_coef = 0;
            }
            
            if x_coef == 0 {
                for x in 0..i32::abs(to_cords[1] - from_cords[1]) + 1 {
                    *grid.get_mut(&(from_cords[0], from_cords[1] + x * y_coef)).unwrap() += 1;
                }
            } 
            else if y_coef == 0 {
                for x in 0..i32::abs(to_cords[0] - from_cords[0]) + 1 {
                    *grid.get_mut(&(from_cords[0] + x * x_coef, from_cords[1])).unwrap() += 1;
                }
            } else {
                for x in 0..i32::abs(to_cords[0] - from_cords[0]) + 1 {
                    *grid.get_mut(&(from_cords[0] + x * x_coef, from_cords[1] + x * y_coef)).unwrap() += 1;
                }
            }
        }

        let mut count = 0;
        for (_, v) in grid.iter() {
            if *v >= 2 {
                count += 1;
            }
        }

        return count;
    }
}
