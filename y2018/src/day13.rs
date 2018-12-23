use aoc_macro::{generator, solution};

type Out<'a> = &'a str;

#[generator]
fn generator(input: &str) -> Out {
    ()
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 0)]
fn part1(input: Out) -> u32 {
    0
}


#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0)]
fn part2(_input: Out) -> u32 {
    0
}

static EXAMPLE: &str = "";
static INPUT: &str = "";
