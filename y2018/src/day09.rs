use aoc_macro::{generator, solution};

type Out<'a> = (u32, u32);

#[generator]
fn generator(input: &str) -> Out {
    ()
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 32,
    expect = 428690)]
fn part1((players, marbles): Out) -> u32 {
    let mut elfs: Vec<u32> = vec![0; players as usize];
    let mut circle: Vec<u32> = vec![0];
    let mut cur: usize = 0;
    let mut next_marble: u32 = 1;
    'outer: loop {
        for elf in &mut elfs {
            if next_marble % 23 == 0 {
                *elf += next_marble;
                let at = if cur >= 7 {
                    cur - 7
                } else {
                    circle.len() - (7 - cur)
                };
                let marble = circle.remove(at);
                *elf += marble;
                cur = at;
            } else {
                let at = (cur + 2) % circle.len();
                circle.insert(at, next_marble);
                cur = at;
            }
            next_marble += 1;

            if next_marble > marbles {
                break 'outer;
            }
            if next_marble % 100_000 == 0 {
                eprintln!("{:?} / {:?}: {:.2}",
                    next_marble, marbles, (next_marble as f64 / marbles as f64));
            }
        }
    }
    elfs.into_iter().max().unwrap()
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 22563)]
fn part2((players, marbles): Out) -> u32 {
    part1((players, marbles * 100))
}

sol_test!(p1: part1((10, 1618)), 8317);
sol_test!(p2: part1((13, 7999)), 146373);
sol_test!(p3: part1((17, 1104)), 2764);
sol_test!(p4: part1((21, 6111)), 54718);
sol_test!(p5: part1((30, 5807)), 37305);

static EXAMPLE: Out = (9, 25);
static INPUT: Out = (405, 71700);
