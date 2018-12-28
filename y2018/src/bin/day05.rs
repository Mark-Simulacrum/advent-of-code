use aoc_macro::{generator, solution};

aoc_macro::day!();

#[generator]
fn generator(input: &str) -> &str {
    input.trim()
}

fn react(mut input: String) -> String {
    let mut made_progress = true;
    while made_progress {
        let start = input.len();
        if input.len() < 2 {
            return input;
        }

        let mut idx = input.len() - 1;
        while idx >= 1 {
            let a = input.as_bytes()[idx] as char;
            let b = input.as_bytes()[idx - 1] as char;
            if a.eq_ignore_ascii_case(&b) {
                let a_upper = a.is_ascii_uppercase();
                let b_upper = b.is_ascii_uppercase();
                if a_upper ^ b_upper {
                    input.drain((idx - 1)..=idx);
                    if idx == 1 {
                        break;
                    } else {
                        idx -= 2;
                        continue;
                    }
                }
            }
            idx -= 1;
        }

        made_progress = start != input.len();
    }
    input
}

#[solution(part1,
    example_input = "dabAcCaCBAcCcaDA",
    example = 10,
    expect = 10598)]
fn part1(input: &str) -> usize {
    react(input.to_string()).len()
}

#[solution(part2,
    example_input = "dabAcCaCBAcCcaDA",
    example = 4,
    expect = 5312)]
fn part2(input: &str) -> usize {
    let mut best = input.len();
    for unit in b'a'..=b'z' {
        let unit = unit as char;
        let stripped = input.replace(unit, "").replace(unit.to_ascii_uppercase(), "");
        best = std::cmp::min(best, react(stripped).len());
    }
    best
}

static INPUT: &str = include_str!("day05.input");
