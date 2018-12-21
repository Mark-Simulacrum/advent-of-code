use aoc_macro::{generator, solution};

type Out = (&'static str, Vec<(&'static str, bool)>);

#[generator]
fn generator((input, patterns): (&'static str, &'static str)) -> Out {
    let input = input.trim();
    let patterns = patterns.trim().lines().map(|line| {
        let mut s = line.split(" => ");
        let pattern = s.next().unwrap();
        let out = s.next().unwrap().as_bytes()[0] == b'#';
        (pattern, out)
    }).collect::<Vec<_>>();

    (input, patterns)
}

fn count(input: &str, padding: usize) -> isize {
    let start: isize = -(padding as isize);
    let mut sum = 0;
    for (idx, ch) in input.chars().enumerate() {
        if ch == '#' {
            sum += start + idx as isize;
        }
    }
    sum
}

fn cycle(
    input: &str,
    patterns: &[(&'static str, bool)],
) -> String {
    let mut new_input = ".".repeat(input.len());
    for &(ref pattern, out) in patterns {
        let mut start = 0;
        while let Some(idx) = input[start..].find(pattern) {
            let idx = start + idx;
            new_input.replace_range((idx + 2)..(idx + 3), if out {
                "#"
            } else {
                "."
            });
            start = idx + 1;
        }
    }
    new_input
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 325)]
fn part1((input, patterns): Out) -> isize {
    let padding = ".".repeat(50);
    let mut input = format!("{}{}{}", padding, input, padding);
    for _ in 1..=20 {
        input = cycle(&input, &patterns);
    }
    count(&input, padding.len())
}


#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0)]
fn part2((input, patterns): Out) -> isize {
    let padding = ".".repeat(50);
    let mut input = format!("{}{}{}", padding, input, padding);
    // 50 billion cycles
    for i in 1..=50_000_000_000u64 {
        input = cycle(&input, &patterns);
        if i % 1_000_000 == 0 {
            eprintln!("{} / {} => {:.2}",
                i,
                50_000_000_000u64,
                (i as f64 /
                50_000_000_000.0) * 100.0);
        }
    }
    count(&input, padding.len())
}

static EXAMPLE: (&str, &str) = (
"#..#.#..##......###...###..............",
"
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
);

static INPUT: (&str, &str) = (
"##.#.#.##..#....######..#..#...#.#..#.#.#..###.#.#.#..#..###.##.#..#.##.##.#.####..##...##..#..##.#.",
"
...## => #
#.#.# => #
.###. => #
#.#.. => .
.#..# => #
#..#. => #
..##. => .
....# => .
#.... => .
###.. => #
.#### => #
###.# => .
#..## => #
..... => .
##.## => #
####. => .
##.#. => .
#...# => .
##### => .
..#.. => .
.#.#. => .
#.### => .
.##.# => .
..#.# => .
.#.## => #
...#. => .
##... => #
##..# => #
.##.. => .
.#... => #
#.##. => #
..### => .
");
