use std::fmt;
use advent_of_code::{Grid, BitVec};

pub fn part1(s: &str) -> u32 {
    let (iterations, states) = parse(s);
    let mut position = 0;
    let mut state_idx = 0;
    let mut tape = Grid::<bool, BitVec>::new();
    for _ in 0..iterations {
        let state = states[state_idx];
        let (value, direction, next) = if tape.get(position, 0) {
            state.one
        } else {
            state.zero
        };
        tape.set(position, 0, value);
        position += direction;
        state_idx = next;
    }
    tape.count_set()
}

pub fn part2(_: &str) -> usize {
    0
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 3);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 2526);
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct State {
    // value, direction, next_state
    zero: (bool, isize, usize),
    one: (bool, isize, usize),
}

impl State {
    fn invalid() -> State {
        State {
            zero: (false, 1000, 1000),
            one: (false, 1000, 1000),
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "0: write {}, go {}, next {}", self.zero.0 as u8, self.zero.1, self.zero.2)?;
        writeln!(f, "1: write {}, go {}, next {}", self.one.0 as u8, self.one.1, self.one.2)?;
        write!(f, "}}")
    }
}

fn parse(s: &str) -> (usize, Vec<State>) {
    let lines = s.trim().lines().collect::<Vec<_>>();
    assert_eq!(lines[0], "Begin in state A.");
    let start = lines[1].find("after").unwrap() + "after ".len();
    let till = lines[1][start..lines[1].find(" steps").unwrap()].parse::<usize>().unwrap();

    let mut states = Vec::new();
    let mut state = State::invalid();
    let mut is_zero = true;
    for line in &lines[3..] {
        if line.starts_with("In state ") && state != State::invalid() {
            states.push(state);
            state = State::invalid();
            is_zero = true;
        }
        if line.contains("current value is 0") {
            is_zero = true;
            continue;
        }
        if line.contains("current value is 1") {
            is_zero = false;
            continue;
        }
        let to = if is_zero { &mut state.zero } else { &mut state.one };
        if line.contains("Write the value") {
            to.0 = line.as_bytes()[22] == b'1';
        }
        if line.contains("Move one slot") {
            if line.contains("right") { to.1 = 1; } else { to.1 = -1; }
        }
        if line.contains("Continue with state") {
            to.2 = (line.as_bytes()[26] - b'A') as usize;
        }
    }
    states.push(state);
    (till, states)
}

pub static INPUT: &str = "
Begin in state A.
Perform a diagnostic checksum after 12656374 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state C.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state D.

In state C:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state D.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state C.

In state D:
  If the current value is 0:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state E.

In state E:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state C.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state F.

In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

#[cfg(test)]
static EXAMPLE: &str = "
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";
