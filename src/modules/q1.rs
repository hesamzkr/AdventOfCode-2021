use crate::modules::base;

pub fn main() {
    let q = Q1 { number: 1 };
    q.run();
}

struct Q1 {
    number: u32,
}

impl Q1 {
    pub fn run(&self) {}

    pub fn part_one(&self) {}

    pub fn part_two(&self) {}
}

// impl base::Question for Q1 {}
