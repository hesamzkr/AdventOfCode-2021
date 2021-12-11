use std::fs;

pub struct Solution {
    number: u32,
}

impl Solution {
    pub fn run() {
        let q = Solution { number: 10 };
        println!("Question {}", q.number);
        let input_str = fs::read_to_string("./inputs/q10.txt").expect("Something went wrong");

        let input: Vec<&str> = input_str.split("\r\n").collect();


        let part_a = q.part_one(&input);
        println!("Part A: {}", part_a.0);
        println!("Part B: {}", q.part_two(&input, part_a.1));
    }

    fn part_one(&self, lines: &Vec<&str>) -> (u32, Vec<usize>) {
        let mut error_score = 0;
        let mut corrupted_lines: Vec<usize> = vec![];

        for i in 0..lines.len() {
            let mut expected: Vec<char> = vec![];
            for ch in lines[i].chars() {
                if "([{<".contains(ch) {
                    expected.push(")]}>".chars().nth("([{<".chars().position(|s| s == ch).unwrap()).unwrap());
                } else if ch != expected[expected.len() - 1] {
                    match ch {
                        ')' => error_score += 3,
                        ']' => error_score += 57,
                        '}' => error_score += 1197,
                        '>' => error_score += 25137,
                        _ => println!("Invalid char"),
                    }

                    corrupted_lines.push(i);
                    break;
                } else {
                    expected.pop();
                }
            }
        }


        return (error_score, corrupted_lines);
    }

    fn part_two(&self, lines: &Vec<&str>, corrupted_lines: Vec<usize>) -> u64 {
        
        let mut scores: Vec<u64> = vec![];

        for i in 0..lines.len() {
            if corrupted_lines.contains(&i) {
                continue;
            }

            let mut expected: Vec<char> = vec![];
            for ch in lines[i].chars() {
                if "([{<".contains(ch) {
                    expected.push(")]}>".chars().nth("([{<".chars().position(|s| s == ch).unwrap()).unwrap());
                } else {
                    expected.pop();
                }
            }

            let mut score = 0;
            expected.reverse();
            for x in expected {
                score *= 5;
                match x {
                    ')' => score += 1,
                    ']' => score += 2,
                    '}' => score += 3,
                    '>' => score += 4,
                    _ => println!("Invalid char"),
                }
            }

            scores.push(score);

        }

        scores.sort();
        return scores[((scores.len() / 2) as f64).ceil() as usize];
    }
}