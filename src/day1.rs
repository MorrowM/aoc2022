use aoc2022;

fn main() {
    aoc2022::run(1, &parse_ints, &part1, &part2)
}

fn part1(elves: Vec<Vec<i64>>) -> i64 {
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn part2(elves: Vec<Vec<i64>>) -> i64 {
    let mut sums = elves
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<i64>>();
    sums.sort_by(|a, b| b.cmp(a));
    sums.iter().take(3).sum()
}

fn parse_ints(input: String) -> Option<Vec<Vec<i64>>> {
    Some(
        input
            .split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect()
            })
            .collect(),
    )
}
