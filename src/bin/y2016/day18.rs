use std::iter;

use itertools::Itertools;

use advent_of_code::BitVec;

fn parse(s: &str) -> BitVec {
    s.as_bytes()
        .iter()
        .map(|b| match b {
            b'.' => false,
            b'^' => true,
            _ => unreachable!("unexpected char {:?}", *b as char),
        })
        .collect()
}

fn to_next_row(previous_row: impl Iterator<Item = bool>) -> BitVec {
    iter::once(false)
        .chain(previous_row)
        .chain(iter::once(false))
        .tuple_windows()
        .map(|(a, b, c)| match (a, b, c) {
            (true, true, false) => true,
            (false, true, true) => true,
            (true, false, false) => true,
            (false, false, true) => true,
            _ => false,
        })
        .collect::<BitVec>()
}

fn count_safe(s: &str, row_count: usize) -> usize {
    let tiles = parse(s);
    let mut rows = 1;
    let mut current = tiles;
    let mut safe = 0;

    safe += current.count_zeros();
    while rows < row_count {
        let next = to_next_row(current.iter());
        safe += next.count_zeros();
        current = next;
        rows += 1;
    }
    safe
}

pub fn part1(s: &str) -> usize {
    count_safe(s, 40)
}

pub fn part2(s: &str) -> usize {
    count_safe(s, 400_000)
}

#[test]
fn part1_1() {
    assert_eq!(part1(INPUT), 2013);
}

#[test]
fn part2_1() {
    assert_eq!(part2(INPUT), 20006289);
}

pub static INPUT: &str = ".^^^.^.^^^.^.......^^.^^^^.^^^^..^^^^^.^.^^^..^^.^.^^..^.^..^^...^.^^.^^^...^^.^.^^^..^^^^.....^....";
