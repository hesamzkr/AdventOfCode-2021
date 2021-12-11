use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 9 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q9.txt").expect("Something went wrong");

        let input: Vec<Vec<u32>> = input_str.split("\r\n").map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect();


        let part_a = q.part_one(&input);
        println!("Part A: {}", part_a.0);
        println!("Part B: {}", q.part_two(&input, part_a.1));
    }

    fn part_one(&self, input: &Vec<Vec<u32>>) -> (u32, Vec<[usize; 2]>) {
        let mut sum = 0;
        let mut low_points: Vec<[usize; 2]> = vec![];
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                let x = input[i][j];
                let mut num_of_checks = 4;
                let mut check_count = 0;

                if j == input[0].len() - 1 || j == 0 {
                    num_of_checks -= 1;
                }
                if i == input.len() - 1 || i == 0 {
                    num_of_checks -= 1;
                }

                if j != input[0].len() - 1 && x < input[i][j + 1] {
                    check_count += 1;
                }
                if j != 0 && x < input[i][j - 1] {
                    check_count += 1;
                }
                if i != input.len() - 1 && x < input[i + 1][j] {
                    check_count += 1;
                }
                if i != 0 && x < input[i - 1][j] {
                    check_count += 1
                }

                if check_count == num_of_checks {
                    sum += x + 1;
                    low_points.push([i, j]);
                }                
            }
        }

        return (sum, low_points);
    }

    fn part_two(&self, input: &Vec<Vec<u32>>, low_points: Vec<[usize; 2]>) -> u32 {
        let mut basin_sizes: Vec<u32> = vec![];

        for low in low_points {
            basin_sizes.push(self.basin_size(input, low[1], low[0], vec![]).0);
        }

        basin_sizes.sort();
        let mut output = 1;
        for i in 1..4 {
            output *= basin_sizes[basin_sizes.len() - i];
        }

        return output;
    }



    fn basin_size(&self, input: &Vec<Vec<u32>>, x: usize, y: usize, mut visited_coords: Vec<(usize, usize)>) -> (u32, Vec<(usize, usize)>) {
        let height = input[y][x];
        visited_coords.push((x, y));
        let mut total_size = 1;

        if x != 0 && !visited_coords.contains(&(x-1, y)) && height < input[y][x-1] && input[y][x-1] != 9 {
            let (size, coords) = self.basin_size(input, x-1, y, visited_coords);
            visited_coords = coords;
            total_size += size;
        }

        if x != input[0].len() - 1 && !visited_coords.contains(&(x+1, y)) && height < input[y][x+1] && input[y][x+1] != 9 {
            let (size, coords) = self.basin_size(input, x+1, y, visited_coords);
            visited_coords = coords;
            total_size += size;
        }

        if y != 0 && !visited_coords.contains(&(x, y-1)) && height < input[y-1][x] && input[y-1][x] != 9 {
            let (size, coords) = self.basin_size(input, x, y-1, visited_coords);
            visited_coords = coords;
            total_size += size;
        }

        if y != input.len() - 1 && !visited_coords.contains(&(x, y+1)) && height < input[y+1][x] && input[y+1][x] != 9 {
            let (size, coords) = self.basin_size(input, x, y+1, visited_coords);
            visited_coords = coords;
            total_size += size;
        }

        return (total_size, visited_coords);
    }
}
