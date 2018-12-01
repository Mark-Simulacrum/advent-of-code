use aoc_macro::solution;

fn generator(input: &str) -> &str {
    input
}

#[solution(part1,
    example_input = "100",
    example = 100,
    expect = 10)]
fn part1(input: &str) -> usize {
    input.parse().unwrap()
}

#[solution(part2,
    example_input = "10",
    example = 10)]
fn part2(input: &str) -> usize {
    input.parse().unwrap()
}

static INPUT: &str = "10";
