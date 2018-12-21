use aoc_macro::{generator, solution};

type Out<'a> = &'a str;

#[generator]
fn generator(_input: &str) -> Out {
    eprintln!("This day is not implemented in Rust due to requirement of -- essentially -- OCR");
    eprintln!("See https://docs.google.com/spreadsheets/d/1whrmqDXiUvX2bXp4qoyZMX68JQ0IzY5JAa3yIuphpg8/edit");
    std::process::exit(1);
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 0)]
fn part1(_input: Out) -> u32 {
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
