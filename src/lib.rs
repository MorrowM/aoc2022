use std::{fmt::Display, fs, path::Path};

pub fn run<Input, Output>(
    day: i32,
    parser: &dyn Fn(String) -> Option<Input>,
    part1: &dyn Fn(Input) -> Output,
    part2: &dyn Fn(Input) -> Output,
) where
    Output: Display,
    Input: Clone,
{
    let contents =
        fs::read_to_string(Path::new(&format!("data/day{day}.txt"))).expect("Could not read file!");
    let input = parser(contents).expect("Couldn't parse input!");
    let out1 = part1(input.clone());
    println!("Part 1: {out1}");
    let out2 = part2(input);
    println!("Part 2: {out2}");
}

pub fn not_implemented<T>() -> T {
    panic!("Not implemented!")
}

pub fn modulus(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}
