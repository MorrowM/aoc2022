use aoc2022::*;

fn main() {
    run(5, &parse, &part1, &part2)
}

#[derive(Clone, Debug)]
struct Instr {
    amount: usize,
    from: usize,
    to: usize,
}

fn part1(input: (Vec<String>, Vec<Instr>)) -> String {
    do_the_thing(input, true)
}

fn part2(input: (Vec<String>, Vec<Instr>)) -> String {
    do_the_thing(input, false)
}

fn do_the_thing((mut stacks, instrs): (Vec<String>, Vec<Instr>), reverse: bool) -> String {
    for instr in instrs {
        let fromlen = stacks[instr.from].len();
        let split_ind = fromlen - instr.amount;

        let (_, diff) = stacks[instr.from].split_at(split_ind);

        let cs = if reverse {
            diff.chars().rev().collect::<String>()
        } else {
            diff.to_string()
        };

        stacks[instr.to].extend(cs.chars());
        stacks[instr.from].drain(split_ind..fromlen).count();
    }

    stacks
        .iter()
        .filter_map(|s| s.chars().last())
        .collect::<String>()
}

// Parsing

fn parse(input: String) -> Option<(Vec<String>, Vec<Instr>)> {
    let (stack_str, instrs_str) = input.split_once("\n\n")?;
    let mut stack_str_lines = stack_str.lines().collect::<Vec<_>>();
    let stack_numbers = stack_str_lines.pop()?;
    let num_stacks: usize = stack_numbers
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .max()?;

    let mut stacks: Vec<String> = (0..num_stacks).map(|_| "".to_string()).collect();
    for line in stack_str.lines() {
        for i in 0..num_stacks {
            let c = line.chars().nth(4 * i + 1)?;
            if c.is_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    for i in 0..num_stacks {
        stacks[i] = stacks[i].chars().rev().collect();
    }

    let instrs = instrs_str
        .lines()
        .map(|line| {
            let words = line.split_ascii_whitespace().collect::<Vec<_>>();
            Instr {
                amount: words[1].parse().unwrap(),
                from: words[3].parse::<usize>().unwrap() - 1,
                to: words[5].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();

    Some((stacks, instrs))
}
