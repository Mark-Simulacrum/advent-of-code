#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Step {
    Right,
    Up,
    Left,
    Down,
}

impl Step {
    fn spiral(self) -> Step {
        match self {
            Step::Down => Step::Right,
            Step::Right => Step::Up,
            Step::Up => Step::Left,
            Step::Left => Step::Down,
        }
    }
}

pub fn part1(n: usize) -> usize {
    if n == 1 { return 0; }
    let spiral = (((n as f64).sqrt() - 1.0) / 2.0).ceil() as usize;
    let top_right = 4*spiral.pow(2) - 2 * spiral + 1;
    let bot_left  = 4*spiral.pow(2) + 2 * spiral + 1;
    let top_left  = 4*spiral.pow(2) + 1;
    let mid = (bot_left - top_left) / 2;
    let center_left = top_left + mid;
    let center_top = top_left - mid;
    let center_right = top_right - mid;
    let center_bot = bot_left + mid;
    let distance_to_center = [center_left, center_top, center_right, center_bot].iter()
        .map(|x| n.checked_sub(*x).unwrap_or_else(|| x -n ))
        .min().unwrap();
    spiral + distance_to_center
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
        let mut value = 0;
        let to_find = &[
            position.go(Step::Right),
            position.go(Step::Up),
            position.go(Step::Down),
            position.go(Step::Up).go(Step::Left),
            position.go(Step::Up).go(Step::Right),
            position.go(Step::Down).go(Step::Left),
            position.go(Step::Down).go(Step::Right),
            position.go(Step::Left),
        ];
        for x in &squares {
            if to_find.contains(&x.0) {
                value += x.1;
            }
        }
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
            }
            strides_run += 1;
            last_step = last_step.spiral();
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
fn part1_actual() {
    assert_eq!(part1(INPUT), 552);
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

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 330785);
}

pub const INPUT: usize = 325489;
