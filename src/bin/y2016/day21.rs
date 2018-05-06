use std::str;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn parse(s: &str) -> Vec<Instruction> {
    s.trim()
        .lines()
        .map(|line| {
            if line.starts_with("swap position ") {
                let from = line["swap position ".len()..][..1].parse().unwrap();
                let to = line["swap position x with position ".len()..][..1]
                    .parse()
                    .unwrap();
                Instruction::SwapPositions(from, to)
            } else if line.starts_with("swap letter") {
                let from = line["swap letter ".len()..].as_bytes()[0];
                let to = line["swap letter x with letter ".len()..].as_bytes()[0];
                Instruction::SwapLetters(from, to)
            } else if line.starts_with("rotate left") {
                Instruction::RotateLeft(line["rotate left ".len()..][..1].parse().unwrap())
            } else if line.starts_with("rotate right") {
                Instruction::RotateRight(line["rotate right ".len()..][..1].parse().unwrap())
            } else if line.starts_with("rotate based") {
                Instruction::RotateLetter(
                    line["rotate based on position of letter ".len()..].as_bytes()[0],
                )
            } else if line.starts_with("reverse positions") {
                let from = line["reverse positions ".len()..][..1].parse().unwrap();
                let to = line["reverse positions x through ".len()..][..1]
                    .parse()
                    .unwrap();
                Instruction::Reverse(from, to)
            } else if line.starts_with("move position") {
                let from = line["move position ".len()..][..1].parse().unwrap();
                let to = line["move position x to position ".len()..][..1]
                    .parse()
                    .unwrap();
                Instruction::Move(from, to)
            } else {
                unreachable!("unexpected instruction: {:?}", line);
            }
        })
        .collect()
}

fn rotate_based(input: &mut [u8], l: u8) {
    let idx = input.iter().position(|c| *c == l).unwrap();
    input.rotate_right(1 + idx);
    if idx >= 4 {
        input.rotate_right(1);
    }
}

pub fn part1(s: &str) -> String {
    let mut input = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
    let instructions = parse(s);
    for instruction in instructions {
        match instruction {
            Instruction::SwapPositions(a, b) => {
                input.swap(a, b);
            }
            Instruction::SwapLetters(a, b) => {
                let a_idx = input.iter().position(|c| *c == a).unwrap();
                let b_idx = input.iter().position(|c| *c == b).unwrap();
                input.swap(a_idx, b_idx);
            }
            Instruction::RotateLeft(a) => {
                input.rotate_left(a);
            }
            Instruction::RotateRight(a) => {
                input.rotate_right(a);
            }
            Instruction::RotateLetter(l) => {
                rotate_based(&mut input, l);
            }
            Instruction::Reverse(a, b) => {
                input[a..=b].reverse();
            }
            Instruction::Move(from, to) => {
                if from < to {
                    input[from..=to].rotate_left(1);
                } else {
                    input[to..=from].rotate_right(1);
                }
            }
        }
    }
    str::from_utf8(&input[..]).unwrap().to_string()
}

pub fn part2(s: &str) -> String {
    let mut input = [b'f', b'b', b'g', b'd', b'c', b'e', b'a', b'h'];
    let instructions = parse(s);
    for instruction in instructions.into_iter().rev() {
        match instruction {
            Instruction::SwapPositions(a, b) => {
                input.swap(a, b);
            }
            Instruction::SwapLetters(a, b) => {
                let a_idx = input.iter().position(|c| *c == a).unwrap();
                let b_idx = input.iter().position(|c| *c == b).unwrap();
                input.swap(a_idx, b_idx);
            }
            Instruction::RotateLeft(a) => {
                input.rotate_right(a);
            }
            Instruction::RotateRight(a) => {
                input.rotate_left(a);
            }
            Instruction::RotateLetter(l) => {
                let mut copy = input;
                loop {
                    let mut copy2 = copy;
                    rotate_based(&mut copy2, l);
                    // that is, rotating this copy produced our original input
                    if copy2 == input {
                        input = copy;
                        break;
                    }
                    copy.rotate_left(1);
                }
            }
            Instruction::Reverse(a, b) => {
                input[a..=b].reverse();
            }
            Instruction::Move(from, to) => {
                if from < to {
                    input[from..=to].rotate_right(1);
                } else {
                    input[to..=from].rotate_left(1);
                }
            }
        }
    }
    str::from_utf8(&input[..]).unwrap().to_string()
}

pub static INPUT: &str = include_str!("day21.input");
