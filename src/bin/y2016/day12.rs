#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    pub fn parse(s: &str) -> Register {
        match s.chars().nth(0).unwrap() {
            'a' => Register::A,
            'b' => Register::B,
            'c' => Register::C,
            'd' => Register::D,
            invalid => unreachable!(
                "unexpected input: {:?}, expected register in {:?}",
                invalid, s
            ),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Register(Register),
    Constant(i32),
}

impl Value {
    pub fn parse(s: &str) -> Value {
        let first_word = s.trim_left().split(' ').next().unwrap();
        match first_word.parse::<i32>() {
            Ok(value) => Value::Constant(value),
            Err(_) => Value::Register(Register::parse(first_word)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Copy(Value, Value),
    Jump(Value, Value),
    Increment(Register),
    Decrement(Register),
    Toggle(Register),
    Out(Value),
}

impl Instruction {
    pub fn parse(s: &str) -> Instruction {
        let second_word = || s.split(' ').nth(1).unwrap();
        let third_word = || s.split(' ').nth(2).unwrap();
        match &s[0..3] {
            "cpy" => Instruction::Copy(
                Value::parse(&s[3..]),
                Value::Register(Register::parse(third_word())),
            ),
            "inc" => Instruction::Increment(Register::parse(second_word())),
            "dec" => Instruction::Decrement(Register::parse(second_word())),
            "tgl" => Instruction::Toggle(Register::parse(second_word())),
            "jnz" => Instruction::Jump(Value::parse(&s[3..]), Value::parse(third_word())),
            "out" => Instruction::Out(Value::parse(second_word())),
            invalid => unreachable!("invalid instruction: {}", invalid),
        }
    }
}

#[derive(Debug)]
pub struct Memory {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

impl Memory {
    pub fn resolve(&self, value: Value) -> i32 {
        match value {
            Value::Constant(v) => v,
            Value::Register(Register::A) => self.a,
            Value::Register(Register::B) => self.b,
            Value::Register(Register::C) => self.c,
            Value::Register(Register::D) => self.d,
        }
    }

    pub fn get_mut(&mut self, to: Register) -> &mut i32 {
        match to {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }

    pub fn eval(&mut self, instr: Instruction) -> i32 {
        match instr {
            Instruction::Copy(value, Value::Register(to)) => {
                *self.get_mut(to) = self.resolve(value)
            }
            Instruction::Copy(_, Value::Constant(_)) => {}
            Instruction::Increment(reg) => *self.get_mut(reg) += 1,
            Instruction::Decrement(reg) => *self.get_mut(reg) -= 1,
            Instruction::Jump(value, offset) => {
                if self.resolve(value) != 0 {
                    return self.resolve(offset);
                }
            }
            Instruction::Toggle(_) => unreachable!(),
            Instruction::Out(_) => unreachable!(),
        }

        1
    }
}

pub fn part1(s: &str) -> i32 {
    let instrs = s.trim()
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let mut mem = Memory {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };

    let mut idx = 0i32;
    while (idx as usize) < instrs.len() {
        idx += mem.eval(instrs[idx as usize]);
    }

    mem.a
}

#[test]
fn part1_1() {
    assert_eq!(
        part1(
            "cpy 41 a
    inc a
    inc a
    dec a
    jnz a 2
    dec a"
        ),
        42
    );
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 317993);
}

pub fn part2(s: &str) -> i32 {
    let instrs = s.trim()
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Vec<_>>();

    let mut mem = Memory {
        a: 0,
        b: 0,
        c: 1,
        d: 0,
    };

    let mut idx = 0i32;
    while (idx as usize) < instrs.len() {
        idx += mem.eval(instrs[idx as usize]);
    }

    mem.a
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 9227647);
}

pub static INPUT: &str = "
cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 13 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5
";
