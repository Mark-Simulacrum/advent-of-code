use advent_of_code::Parser;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::North => Direction::West,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn go(&mut self, n: usize, dir: Direction) {
        let n = n as isize;
        match dir {
            Direction::North => self.y += n,
            Direction::South => self.y -= n,
            Direction::East => self.x += n,
            Direction::West => self.x -= n,
        }
    }
}

pub fn part1(s: &str) -> usize {
    let mut dir = Direction::North;
    let mut pos = Position { x: 0, y: 0 };

    for s in s.split(", ") {
        if s.as_bytes()[0] == b'R' {
            dir = dir.turn_right();
        } else {
            dir = dir.turn_left();
        }
        let mut p = Parser::new(s[1..].as_bytes());
        let n = p.consume_number().expect("parse num") as usize;
        pos.go(n, dir);
    }

    pos.x.abs() as usize + pos.y.abs() as usize
}

pub fn part2(s: &str) -> usize {
    let mut dir = Direction::North;
    let mut visited = HashSet::new();
    let mut pos = Position { x: 0, y: 0 };
    visited.insert(pos);

    for s in s.split(", ") {
        if s.as_bytes()[0] == b'R' {
            dir = dir.turn_right();
        } else {
            dir = dir.turn_left();
        }
        let mut p = Parser::new(s[1..].as_bytes());
        let n = p.consume_number().expect("parse num") as usize;
        for _ in 0..n {
            pos.go(1, dir);
            if !visited.insert(pos) {
                return pos.x.abs() as usize + pos.y.abs() as usize;
            }
        }
    }

    panic!("did not visit any location twice");
}

#[test]
fn part1_1() {
    assert_eq!(part1("R2, L3"), 5);
}

#[test]
fn part1_2() {
    assert_eq!(part1("R2, R2, R2"), 2);
}

#[test]
fn part1_3() {
    assert_eq!(part1("R5, L5, R5, R3"), 12);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 241);
}

#[test]
fn part2_1() {
    assert_eq!(part2("R8, R4, R4, R8"), 4);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 116);
}

pub const INPUT: &str = "R1, R1, R3, R1, R1, L2, R5, L2, R5, R1, R4, L2, R3, L3, R4, L5, R4, R4, R1, L5, L4, R5, R3, L1, R4, R3, L2, L1, R3, L4, R3, L2, R5, R190, R3, R5, L5, L1, R54, L3, L4, L1, R4, R1, R3, L1, L1, R2, L2, R2, R5, L3, R4, R76, L3, R4, R191, R5, R5, L5, L4, L5, L3, R1, R3, R2, L2, L2, L4, L5, L4, R5, R4, R4, R2, R3, R4, L3, L2, R5, R3, L2, L1, R2, L3, R2, L1, L1, R1, L3, R5, L5, L1, L2, R5, R3, L3, R3, R5, R2, R5, R5, L5, L5, R2, L3, L5, L2, L1, R2, R2, L2, R2, L3, L2, R3, L5, R4, L4, L5, R3, L4, R1, R3, R2, R4, L2, L3, R2, L5, R5, R4, L2, R4, L1, L3, L1, L3, R1, R2, R1, L5, R5, R3, L3, L3, L2, R4, R2, L5, L1, L1, L5, L4, L1, L1, R1";
