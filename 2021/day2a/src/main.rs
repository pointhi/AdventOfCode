use std::borrow::Borrow;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please specify filename");

    println!("Read file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    contents.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        let number = split
            .get(1)
            .unwrap()
            .parse::<i32>()
            .expect("Cannot convert part to i32");
        match split.get(0).unwrap().borrow() {
            "forward" => {
                horizontal += number;
                depth += aim * number;
            }
            "up" => aim -= number,
            "down" => aim += number,
            _ => panic!("unexpected command"),
        }
    });

    println!("Horizontal {}", horizontal);
    println!("Depth {}", depth);

    println!("Result {}", horizontal * depth);
}
