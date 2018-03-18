use std::collections::VecDeque;

pub fn to_register(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Register(usize),
    Value(i64),
}

impl Value {
    pub fn resolve(self, registers: &[i64]) -> i64 {
        match self {
            Value::Register(reg) => registers[reg],
            Value::Value(val) => val,
        }
    }

    pub fn parse(s: &str) -> Value {
        match s.parse::<i64>().ok() {
            Some(val) => Value::Value(val),
            None => Value::Register(to_register(s.as_bytes()[0] as char)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Snd(Value),
    Rcv(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Jmp(Value, Value),
}

#[derive(Copy, Clone)]
enum Out {
    Send(i64),
    Receive,
    Jump(isize),
    None,
}

impl Instruction {
    fn exec(self, registers: &mut [i64; 26]) -> Out {
        match self {
            Instruction::Snd(val) => return Out::Send(val.resolve(registers)),
            Instruction::Set(reg, val) => registers[reg] = val.resolve(registers),
            Instruction::Add(reg, val) => registers[reg] += val.resolve(registers),
            Instruction::Mul(reg, val) => registers[reg] *= val.resolve(registers),
            Instruction::Mod(reg, val) => registers[reg] %= val.resolve(registers),
            Instruction::Rcv(val) => {
                if val.resolve(registers) != 0 {
                    return Out::Receive;
                }
            }
            Instruction::Jmp(cond, val) => {
                if cond.resolve(registers) > 0 {
                    return Out::Jump(val.resolve(registers) as isize);
                }
            }
        }
        Out::None
    }
}

fn parse(s: &str) -> Vec<Instruction> {
    s.trim()
        .lines()
        .map(|line| {
            let arg1 = Value::parse(&line[4..5]);
            match &line[0..3] {
                "snd" => Instruction::Snd(arg1),
                "rcv" => Instruction::Rcv(arg1),
                "jgz" => Instruction::Jmp(arg1, Value::parse(&line[6..])),
                val => {
                    let arg1 = to_register(line.as_bytes()[4] as char);
                    match val {
                        "set" => Instruction::Set(arg1, Value::parse(&line[6..])),
                        "add" => Instruction::Add(arg1, Value::parse(&line[6..])),
                        "mul" => Instruction::Mul(arg1, Value::parse(&line[6..])),
                        "mod" => Instruction::Mod(arg1, Value::parse(&line[6..])),
                        _ => unreachable!("unexpected instruction: {}", line),
                    }
                }
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1(s: &str) -> i64 {
    let instructions = parse(s);
    let mut registers = [0; 26];
    let mut last_sound = None;
    let mut pos = 0;
    loop {
        let instr = instructions[pos];
        pos += 1;
        match instr.exec(&mut registers) {
            Out::None => {}
            Out::Receive => return last_sound.unwrap(),
            Out::Send(x) => last_sound = Some(x),
            Out::Jump(off) => pos = ((pos as isize) + off - 1) as usize,
        }
    }
}

struct Program<'a> {
    registers: [i64; 26],
    instructions: &'a [Instruction],
    position: usize,
    queue: VecDeque<i64>,
    sent: usize,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a [Instruction], id: i64) -> Program<'a> {
        let mut registers = [0; 26];
        registers[to_register('p')] = id;
        Program {
            instructions,
            registers,
            position: 0,
            queue: VecDeque::new(),
            sent: 0,
        }
    }

    // returns if we made progress
    fn execute(&mut self, input: &mut VecDeque<i64>) -> bool {
        let start = self.position;
        if self.position == self.instructions.len() {
            return false;
        }
        loop {
            let instr = self.instructions[self.position];
            self.position += 1;
            match instr {
                Instruction::Snd(val) => {
                    self.queue.push_back(val.resolve(&self.registers));
                    self.sent += 1;
                    break;
                }
                Instruction::Rcv(Value::Register(reg)) => {
                    if let Some(input) = input.pop_front() {
                        self.registers[reg] = input;
                    } else {
                        // unable to execute; wait
                        self.position -= 1;
                    }
                    break;
                }
                Instruction::Rcv(Value::Value(_)) => {}
                Instruction::Set(reg, val) => self.registers[reg] = val.resolve(&self.registers),
                Instruction::Add(reg, val) => self.registers[reg] += val.resolve(&self.registers),
                Instruction::Mul(reg, val) => self.registers[reg] *= val.resolve(&self.registers),
                Instruction::Mod(reg, val) => self.registers[reg] %= val.resolve(&self.registers),
                Instruction::Jmp(cond, val) => {
                    if cond.resolve(&self.registers) > 0 {
                        let offset = val.resolve(&self.registers) as isize;
                        self.position -= 1; // counteract the += 1 default above
                        self.position = ((self.position as isize) + offset) as usize;
                    }
                }
            }
        }
        start != self.position
    }
}

pub fn part2(s: &str) -> usize {
    let instructions = parse(s);
    let mut a = Program::new(&instructions, 0);
    let mut b = Program::new(&instructions, 1);
    let mut progress = true;
    while progress {
        progress = false;
        while a.execute(&mut b.queue) {
            progress = true;
        }
        while b.execute(&mut a.queue) {
            progress = true;
        }
    }
    b.sent
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE_1), 4);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 7071);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE_2), 3);
}

#[cfg(test)]
static EXAMPLE_1: &str = "
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";

#[cfg(test)]
static EXAMPLE_2: &str = "
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
";

pub static INPUT: &str = "
set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 826
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19
";
