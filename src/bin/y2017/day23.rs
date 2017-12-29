use day18::{Value, to_register};

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jmp(Value, Value),
}

enum Out {
    Mul,
    None,
    Jmp(isize),
}

impl Instruction {
    fn exec(self, registers: &mut [i64; 8]) -> Out {
        match self {
            Instruction::Set(reg, val) => registers[reg] = val.resolve(registers),
            Instruction::Sub(reg, val) => registers[reg] -= val.resolve(registers),
            Instruction::Mul(reg, val) => {
                registers[reg] *= val.resolve(registers);
                return Out::Mul;
            }
            Instruction::Jmp(cnd, off) => {
                if cnd.resolve(registers) != 0 {
                    return Out::Jmp(off.resolve(registers) as isize);
                }
            }
        }
        Out::None
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    s.trim().lines().map(|line| {
        let arg1 = Value::parse(&line[4..5]);;
        match &line[0..3] {
            "jnz" => Instruction::Jmp(arg1, Value::parse(&line[6..])),
            val => {
                let arg1 = to_register(line.as_bytes()[4] as char);
                match val {
                    "set" => Instruction::Set(arg1, Value::parse(&line[6..])),
                    "sub" => Instruction::Sub(arg1, Value::parse(&line[6..])),
                    "mul" => Instruction::Mul(arg1, Value::parse(&line[6..])),
                    _ => unreachable!("unexpected instruction: {}", line),
                }
            }
        }
    }).collect::<Vec<_>>()
}

pub fn part1(s: &str) -> usize {
    let instructions = parse(s);
    let mut registers = [0; 8];
    let mut times = 0;
    let mut pos = 0;
    while pos < instructions.len() {
        let instr = instructions[pos];
        pos += 1;
        match instr.exec(&mut registers) {
            Out::None => {},
            Out::Mul => times += 1,
            Out::Jmp(off) => pos = ((pos as isize) + off - 1) as usize,
        }
    }
    times
}

/// This is a manually decompiled version of the below assembly, given that a=1.
/// Effectively, we are counting the composite numbers from A to B.
pub fn part2(_: &str) -> i64 {
    let mut b = 105700;
    let mut h = 0;
    let sieve = ::primal::Sieve::new(122700);
    while b <= 122700 {
        if !sieve.is_prime(b) {
            h += 1;
        }
        b += 17;
    }
    h
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 3025);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 915);
}

pub static INPUT: &str = "
set b 57
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23
";
