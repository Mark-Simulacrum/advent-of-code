use day12::{Instruction, Memory};

fn compute(s: &str, eggs: usize) -> usize {
    let mut instrs = s.trim()
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let mut mem = Memory {
        a: eggs as i32,
        b: 0,
        c: 0,
        d: 0,
    };

    let mut idx = 0i32;
    while (idx as usize) < instrs.len() {
        if let Instruction::Toggle(reg) = instrs[idx as usize] {
            let toggle_idx = (idx + *mem.get_mut(reg)) as usize;
            if let Some(instr) = instrs.get(toggle_idx).cloned() {
                instrs[toggle_idx] = match instr {
                    Instruction::Copy(a, b) => Instruction::Jump(a, b),
                    Instruction::Jump(a, b) => Instruction::Copy(a, b),
                    Instruction::Increment(a) => Instruction::Decrement(a),
                    Instruction::Decrement(a) => Instruction::Increment(a),
                    Instruction::Toggle(a) => Instruction::Increment(a),
                };
            }
            idx += 1;
        } else {
            idx += mem.eval(instrs[idx as usize]);
        }
    }

    mem.a as usize
}

pub fn part1(s: &str) -> usize {
    compute(s, 7)
}

pub fn part2(s: &str) -> usize {
    compute(s, 12)
}

pub static INPUT: &str = include_str!("day23.input");
