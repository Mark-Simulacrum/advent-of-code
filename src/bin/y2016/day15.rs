use advent_of_code::modulo_solver::{modulo, Congruence};

fn solve(discs: &[Disc]) -> usize {
    eprintln!("{:?}", discs);
    let congruence = discs
        .iter()
        .enumerate()
        .map(|(i, d)| Congruence {
            a: modulo(-((d.position + i + 1) as i128), d.positions as i128) as i128,
            n: d.positions as i128,
        })
        .fold(None::<Congruence>, |acc, congruence| match acc {
            Some(c) => Some(c.combine(congruence).unwrap()),
            None => Some(congruence),
        });
    congruence.unwrap().a as usize
}

pub fn part1(discs: &[Disc]) -> usize {
    solve(&discs[..discs.len() - 1])
}

pub fn part2(discs: &[Disc]) -> usize {
    solve(&discs)
}

#[derive(Debug)]
pub struct Disc {
    positions: usize,
    position: usize,
}

#[test]
fn part1_1() {
    assert_eq!(solve(EXAMPLE), 5);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 121834);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 3208099);
}

#[cfg(test)]
static EXAMPLE: &[Disc] = &[
    //Disc #1 has 5 positions; at time=0, it is at position 4.
    //Disc #2 has 2 positions; at time=0, it is at position 1.
    Disc {
        positions: 5,
        position: 4,
    },
    Disc {
        positions: 2,
        position: 1,
    },
];

pub static INPUT: &[Disc] = &[
    Disc {
        positions: 7,
        position: 0,
    },
    Disc {
        positions: 13,
        position: 0,
    },
    Disc {
        positions: 3,
        position: 2,
    },
    Disc {
        positions: 5,
        position: 2,
    },
    Disc {
        positions: 17,
        position: 0,
    },
    Disc {
        positions: 19,
        position: 7,
    },
    // only for part 2:
    Disc {
        positions: 11,
        position: 0,
    },
];
