use aoc_macro::{generator, solution};
use hashbrown::HashSet;

aoc_macro::day!();

use y2018::device::op_codes::*;
use y2018::device::{RawOp, Registers};

#[derive(Copy, Clone, Debug)]
struct Sample {
    before: Registers,
    op: RawOp,
    after: Registers,
}

impl Sample {
    fn check(&self, op: fn(RawOp, &mut Registers)) -> bool {
        let mut regs = self.before;
        op(self.op, &mut regs);
        regs == self.after
    }
}

type Out = (Vec<Sample>, Vec<RawOp>);

#[generator]
fn generator((samples_input, raw_program): (&str, &str)) -> Out {
    let mut samples = Vec::new();
    let mut before = None;
    let mut op = None;
    for line in samples_input.trim().lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Before: ") || line.starts_with("After: ") {
            let it = line[9..19].split(", ").map(|d| d.parse::<u32>().unwrap());
            let reg = Registers::from(it);
            if line.contains("Before") {
                before = Some(reg);
            } else {
                samples.push(Sample {
                    before: before.expect("found before"),
                    op: op.expect("found op"),
                    after: reg,
                });
            }
        } else {
            let mut it = line
                .split(' ')
                .filter(|d| !d.is_empty())
                .map(|d| d.parse::<u32>().unwrap());
            op = Some(RawOp {
                code: it.next().unwrap(),
                a: it.next().unwrap(),
                b: it.next().unwrap(),
                c: it.next().unwrap(),
            });
        }
    }
    let mut program = Vec::new();
    for line in raw_program.trim().lines() {
        let mut it = line
            .split(' ')
            .filter(|d| !d.is_empty())
            .map(|d| d.parse::<u32>().unwrap());
        program.push(RawOp {
            code: it.next().unwrap(),
            a: it.next().unwrap(),
            b: it.next().unwrap(),
            c: it.next().unwrap(),
        });
    }
    (samples, program)
}

static OPS: &[fn(RawOp, &mut Registers)] = &[
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 1,
    expect = 517)]
fn part1((input, _): Out) -> u32 {
    let mut count = 0;
    for sample in input {
        let mut worked = 0;
        for op in OPS {
            if sample.check(*op) {
                worked += 1;
            }
        }
        if worked >= 3 {
            count += 1;
        }
    }
    count
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0,
    expect = 667)]
fn part2((input, program): Out, example: bool) -> u32 {
    if example {
        return 0;
    }
    let mut by_code = vec![vec![]; 16];
    for sample in input {
        by_code[sample.op.code as usize].push(sample);
    }
    let mut codes = vec![HashSet::new(); 16];
    for (op_idx, op) in OPS.iter().enumerate() {
        for samples in &by_code {
            if samples.iter().all(|s| s.check(*op)) {
                let code = samples[0].op.code as usize;
                codes[code].insert(op_idx);
            }
        }
    }
    while codes.iter().any(|c| c.len() > 1) {
        let unique = codes
            .iter()
            .filter(|c| c.len() == 1)
            .flat_map(|x| x)
            .cloned()
            .collect::<HashSet<_>>();
        for code in &mut codes {
            if code.len() == 1 {
                continue;
            }
            *code = code.difference(&unique).cloned().collect();
        }
    }
    let mut code_map = vec![99; 16];
    let mut regs = Registers::default();
    for (code, valid_ops) in codes.iter().enumerate() {
        let valid = *valid_ops.iter().next().unwrap();
        code_map[code] = valid;
    }
    for raw in &program {
        OPS[code_map[raw.code as usize]](*raw, &mut regs);
    }
    regs[0]
}

static EXAMPLE: (&str, &str) = (
    "
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
",
    "",
);
static INPUT: (&str, &str) = (
    include_str!("day16-samples.txt"),
    include_str!("day16-program.txt"),
);
