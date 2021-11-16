#![allow(warnings)]
// use std::env;
// use std::fs;
mod modules;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let question_num: Option<&String> = args.get(1);

    // match question_num {
    //     Some(p) => println!("{}", p),
    //     None => println!("Nothing"),
    // }

    modules::q1::main();

    // let text = fs::read_to_string("files/input.txt").expect("Something went wrong");
    // println!("{}", text);
}
