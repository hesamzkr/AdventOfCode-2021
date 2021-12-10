use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 8 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q8.txt").expect("Something went wrong");

        let input: Vec<Vec<&str>> = input_str.split("\n").map(|s| s.split(" | ").collect::<Vec<&str>>()).collect();


        println!("Part A: {}", q.part_one(&input));
        println!("Part B: {}", q.part_two(&input));
    }

    fn part_one(&self, input: &Vec<Vec<&str>>) -> usize {
        let valid_lengths = [2, 3, 4, 7];
        let mut count = 0;
        for i in input {
            for j in i[1].split_whitespace() {
                if valid_lengths.contains(&j.len()) {
                    count += 1;
                } 
            }
        }

        return count;
    }

    fn part_two(&self, displays: &Vec<Vec<&str>>) -> usize {
        let unique_ones = ['1', '7', '4', '8'];
        let mut sum: usize = 0;
        for i in displays {
            let mut display_value: Vec<char> = vec![];
            let inputs: Vec<&str> = i[0].split_whitespace().collect();
            let outputs: Vec<&str> = i[1].split_whitespace().collect();

            // acf
            let seven: Vec<char> = inputs[inputs.iter().position(|&s| s.len() == 3).unwrap()].chars().collect();

            // bcdf
            let four: Vec<char> = inputs[inputs.iter().position(|&s| s.len() == 4).unwrap()].chars().collect();

            for output in outputs {
                if let Some(index) = [2, 3, 4, 7].iter().position(|&s| s == output.len())  {
                    display_value.push(unique_ones[index]);
                } else {
                    if output.len() == 5 {
                        if output.contains(seven[0]) && output.contains(seven[1]) && output.contains(seven[2]) {
                            display_value.push('3');
                        } else {
                            let mut match_count = 0;
                            for f in &four {
                                if output.contains(*f) {
                                    match_count += 1;
                                }
                            }
                            if match_count == 3 {
                                display_value.push('5');
                            } else {
                                display_value.push('2');
                            }
                        }
                    }
                    else if output.len() == 6 {
                        if !(output.contains(seven[0]) && output.contains(seven[1]) && output.contains(seven[2])) {
                            display_value.push('6');
                        } else if output.contains(four[0]) && output.contains(four[1]) && output.contains(four[2]) && output.contains(four[3]) {
                            display_value.push('9');
                        } else {
                            display_value.push('0');
                        }
                    }
                }
            }

            sum += display_value.iter().collect::<String>().parse::<usize>().unwrap();
        }

        return sum;
    }
}
