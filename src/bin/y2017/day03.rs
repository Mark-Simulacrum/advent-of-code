use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Step {
    Right,
    Up,
    Left,
    Down,
}

pub fn part1(n: usize) -> usize {
    let mut position = Position(0, 0);
    let mut i = 1;
    let mut cur = 1;
    let mut strides_run = 0;
    let mut stride = 1;
    let mut last_step = Step::Down;
    while i != n {
        if stride == cur {
            cur = 0;
            if strides_run == 2 {
                stride += 1;
                strides_run = 0;
            }
            strides_run += 1;
            last_step = match last_step {
                Step::Down => Step::Right,
                Step::Right => Step::Up,
                Step::Up => Step::Left,
                Step::Left => Step::Down,
            };
        }
        position = position.go(last_step);
        cur += 1;
        i += 1;
    }
    (position.0.abs() + position.1.abs()) as usize
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Position(i64, i64);

impl Position {
    fn go(mut self, step: Step) -> Self {
        match step {
            Step::Up => self.1 += 1,
            Step::Down => self.1 -= 1,
            Step::Right => self.0 += 1,
            Step::Left => self.0 -= 1,
        }
        self
    }
}

pub fn part2(n: usize) -> usize {
    let mut squares: Vec<(Position, u64)> = Vec::with_capacity(n);
    let mut position = Position(0, 0);
    let mut i = 1;
    let mut cur = 1;
    let mut strides_run = 0;
    let mut stride = 1;
    let mut last_step = Step::Down;
    loop {
        let value =
            squares.iter().find(|v| v.0 == position.go(Step::Left)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Right)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Up)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Down)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Up).go(Step::Left)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Up).go(Step::Right)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Down).go(Step::Left)).map(|x| x.1).unwrap_or(0) +
            squares.iter().find(|v| v.0 == position.go(Step::Down).go(Step::Right)).map(|x| x.1).unwrap_or(0);
        let value = if i == 1 { 1 } else { value };
        squares.push((position, value));
        if value > n as u64 {
            return value as usize;
        }
        if stride == cur {
            cur = 0;
            if strides_run == 2 {
                stride += 1;
                strides_run = 0;
                let len = squares.len() as i64;
                let from_end = (stride * 5) as i64;
                let end = len - from_end;
                let _ = squares.drain(0..(cmp::max(0, end) as usize));
            }
            strides_run += 1;
            last_step = match last_step {
                Step::Down => Step::Right,
                Step::Right => Step::Up,
                Step::Up => Step::Left,
                Step::Left => Step::Down,
            };
        }
        position = position.go(last_step);
        cur += 1;
        i += 1;
    }
}

#[test]
fn part1_1() {
    assert_eq!(part1(1), 0);
}

#[test]
fn part1_2() {
    assert_eq!(part1(12), 3);
}

#[test]
fn part1_3() {
    assert_eq!(part1(23), 2);
}

#[test]
fn part1_4() {
    assert_eq!(part1(1024), 31);
}

#[test]
fn part2_1() {
    assert_eq!(part2(1), 2);
}

#[test]
fn part2_2() {
    assert_eq!(part2(2), 4);
}

#[test]
fn part2_3() {
    assert_eq!(part2(3), 4);
}

#[test]
fn part2_4() {
    assert_eq!(part2(4), 5);
}

#[test]
fn part2_5() {
    assert_eq!(part2(23), 25);
}

pub const INPUT: usize = 325489;
