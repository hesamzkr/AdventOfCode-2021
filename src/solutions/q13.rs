use std::fs;
use std::collections::HashSet;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 13 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q13.txt").expect("Something went wrong");

        let input: Vec<&str> = input_str.split("\r\n").collect();

        let mut coords: Vec<Vec<usize>> = vec![];
        let mut instructions: Vec<(&str, usize)> = vec![];
        let mut coords_finished = false;
        for i in &input {
            if *i == "" {
                coords_finished = true;
                continue;
            }

            if !coords_finished {
                coords.push(i.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>());
            } else {
                let split_instruct: Vec<&str> = i.split(" ").nth(2).unwrap().split("=").collect();
                instructions.push((split_instruct[0], split_instruct[1].parse().unwrap()));
            }
            
        }

        println!("Part A: {}", q.part_one(&coords));
        println!("Part B:\n{}", q.part_two(&coords, instructions));
        
    }

    fn part_one(&self, coords: &Vec<Vec<usize>>) -> u32 {
        let mut grid: Vec<Vec<char>> = vec![];
        for _ in 0..2000 {
            grid.push(".".repeat(2000).chars().collect());
        }

        for c in coords {
            grid[c[1]][c[0]] = '#';
        }

        // x = 655
        for y in 0..grid.len() {
            for x in 0..655 {
                let diff = 655 - x;
                if grid[y][x] == '#' {
                    grid[y][655 + diff] = '#';
                }
            }
        }

        let mut count = 0;
        for y in 0..grid.len() {
            for x in 656..grid[0].len() {
                if grid[y][x] == '#' {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn part_two(&self, coords: &Vec<Vec<usize>>, instructions: Vec<(&str, usize)>) -> String {
        let mut grid: HashSet<(usize, usize)> = HashSet::new();

        for c in coords {
            grid.insert((c[0], c[1]));
        }

        for i in instructions {
            let mut new_grid: HashSet<(usize, usize)> = HashSet::new();
            let fold_num = i.1;

            for g in &grid {
                let x = g.0;
                let y = g.1;

                if i.0 == "x" {
                    if x > fold_num {
                        new_grid.insert((fold_num - (x - fold_num), y));
                    } else {
                        new_grid.insert((x, y));
                    }
                } else {
                    if y > fold_num {
                        new_grid.insert((x, fold_num - (y - fold_num)));
                    } else {
                        new_grid.insert((x, y));
                    }
                }
                
            }

            grid = new_grid;
        }


        let mut height = 0;
        let mut width = 0;
        for i in &grid {
            if i.0 + 1 > width {
                width = i.0 + 1;
            }
    
            if i.1 + 1> height {
                height = i.1 + 1;
            }
        }
        
        let mut output = String::new();
        for i in 0..height {
            for j in 0..width {
                if grid.contains(&(j, i)) {
                    output += "#";
                }
                else {
                    output += " ";
                }
            }
            output += "\n";
        }

        return output;
    }
}
