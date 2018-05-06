use day12::{Instruction, Memory};

pub fn part1(s: &str) -> usize {
    let instrs = s.trim()
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    'main: for start in 0usize.. {
        let mut mem = Memory {
            a: start as i32,
            b: 0,
            c: 0,
            d: 0,
        };

        let mut count = 0;
        let mut cycle = [0, 1].iter().cloned().cycle();

        let mut idx = 0i32;
        while (idx as usize) < instrs.len() {
            if let Instruction::Out(value) = instrs[idx as usize] {
                if cycle.next().unwrap() != mem.resolve(value) {
                    continue 'main;
                } else {
                    count += 1;
                    if count >= 100 {
                        return start;
                    }
                }
                idx += 1;
            } else {
                idx += mem.eval(instrs[idx as usize]);
            }
        }
    }

    unreachable!()
}

pub fn part2(_: &str) -> usize {
    0
}

pub static INPUT: &str = include_str!("day25.input");
