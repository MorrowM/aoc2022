use aoc2022::*;

fn main() {
    run(4, &parse, &part1, &part2)
}

fn part1(elves: Vec<((i64, i64), (i64, i64))>) -> i64 {
    elves
        .iter()
        .filter(|(e1, e2)| contains(e1, e2) || contains(e2, e1))
        .count() as i64
}

fn contains((a, b): &(i64, i64), (c, d): &(i64, i64)) -> bool {
    a <= c && d <= b
}

fn overlaps((a, b): &(i64, i64), (c, d): &(i64, i64)) -> bool {
    b >= c && d >= a
}

fn part2(elves: Vec<((i64, i64), (i64, i64))>) -> i64 {
    elves.iter().filter(|(e1, e2)| overlaps(e1, e2)).count() as i64
}

// Parsing

fn parse(input: String) -> Option<Vec<((i64, i64), (i64, i64))>> {
    Some(
        input
            .lines()
            .map(|s| {
                let (num1, rest) = s.split_once('-').unwrap();
                let (num2, rest) = rest.split_once(',').unwrap();
                let (num3, num4) = rest.split_once('-').unwrap();

                (
                    (num1.parse().unwrap(), num2.parse().unwrap()),
                    (num3.parse().unwrap(), num4.parse().unwrap()),
                )
            })
            .collect(),
    )
}
