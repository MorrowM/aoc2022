use aoc2022::*;

fn main() {
    run(2, &parse_strategy, &part1, &part2)
}

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn point_value(&self) -> i64 {
        *self as i64
    }
}

type Strategy = Vec<(RPS, RPS)>;

fn part1(strat: Strategy) -> i64 {
    strat
        .iter()
        .map(|(you, me)| score_round(me, you) + me.point_value())
        .sum()
}

fn score_round(me: &RPS, you: &RPS) -> i64 {
    match modulus(me.point_value() - you.point_value(), 3) {
        0 => 3, // tie
        1 => 6, // win
        2 => 0, // loss
        _ => panic!("impossible"),
    }
}

fn part2(strat: Strategy) -> i64 {
    strat
        .iter()
        .map(|(you, result)| score_round_part2(you, result))
        .sum()
}

fn score_round_part2(you: &RPS, result: &RPS) -> i64 {
    let (diff, score) = match result {
        RPS::Rock => (2, 0),     // lose
        RPS::Paper => (0, 3),    // draw
        RPS::Scissors => (1, 6), // win
    };

    let me = modulus(you.point_value() - 1 + diff, 3) + 1;
    score + me
}

// Parsing

fn parse_strategy(input: String) -> Option<Strategy> {
    Some(
        input
            .lines()
            .map(|s| parse_round(s.split_whitespace().collect::<Vec<&str>>()))
            .collect(),
    )
}

fn parse_round(strs: Vec<&str>) -> (RPS, RPS) {
    let s1 = strs[0];
    let s2 = strs[1];
    (parse_rps(s1), parse_rps(s2))
}

fn parse_rps(str: &str) -> RPS {
    match str {
        "X" | "A" => RPS::Rock,
        "Y" | "B" => RPS::Paper,
        "Z" | "C" => RPS::Scissors,
        _ => panic!("Bad line: {str}"),
    }
}
