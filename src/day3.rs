use std::collections::HashSet;

use aoc2022::*;

fn main() {
    run(3, &parse, &part1, &part2)
}

fn part1(rucksacks: Vec<String>) -> i64 {
    rucksacks.iter().map(single_rucksack_p1).sum()
}

fn single_rucksack_p1(rucksack: &String) -> i64 {
    let (fst, snd) = split_string(rucksack);
    let fst: HashSet<char> = HashSet::from_iter(fst.chars());
    let snd: HashSet<char> = HashSet::from_iter(snd.chars());
    let common = fst.intersection(&snd).next().unwrap();

    priority(common)
}

fn priority(c: &char) -> i64 {
    if c.is_ascii_lowercase() {
        (*c as i64) - ('a' as i64) + 1
    } else {
        (*c as i64) - ('A' as i64) + 27
    }
}

fn split_string(s: &String) -> (&str, &str) {
    let length = s.len();
    s.split_at(length / 2)
}

fn part2(rucksacks: Vec<String>) -> i64 {
    rucksacks.chunks(3).map(single_group_p2).sum()
}

fn single_group_p2(group: &[String]) -> i64 {
    match group {
        [a, b, c] => {
            let fst: HashSet<char> = HashSet::from_iter(a.chars());
            let snd: HashSet<char> = HashSet::from_iter(b.chars());
            let thd: HashSet<char> = HashSet::from_iter(c.chars());
            let intersect: HashSet<char> = HashSet::from_iter(fst.intersection(&snd).map(|c| *c));
            let common = intersect.intersection(&thd).next().unwrap();
            priority(common)
        }
        _ => panic!("impossible"),
    }
}

// Parsing

fn parse(input: String) -> Option<Vec<String>> {
    Some(input.lines().map(String::from).collect())
}
