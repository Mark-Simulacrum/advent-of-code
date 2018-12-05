use aoc_macro::{generator, solution};

#[generator]
fn generator(input: &str) -> &str {
    input.trim()
}

fn react(mut input: String) -> String {
    let mut made_progress = true;
    while made_progress {
        let bytes = input.as_bytes();
        let mut next = String::with_capacity(bytes.len());
        let mut idx = 0;
        while idx + 1 < bytes.len() {
            let a = bytes[idx] as char;
            let b = bytes[idx + 1] as char;
            let a_upper = a.is_ascii_uppercase();
            let b_upper = b.is_ascii_uppercase();
            if a_upper && !b_upper && a.to_ascii_lowercase() == b {
                idx += 2;
                continue;
            }
            if b_upper && !a_upper && b.to_ascii_lowercase() == a {
                idx += 2;
                continue;
            }
            next.push(a);
            idx += 1;
        }
        // any trailing characters should be pushed on
        while idx < bytes.len() {
            next.push(bytes[idx] as char);
            idx += 1;
        }
        made_progress = next.len() != input.len();
        input = next;
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
