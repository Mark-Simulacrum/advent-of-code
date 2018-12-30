use std::fmt;
use std::ops;

pub mod op_codes;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Registers {
    r: [u32; 6],
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:9?}", self.r)
    }
}

impl<I> From<I> for Registers
where
    I: Iterator<Item = u32>,
{
    fn from(mut it: I) -> Registers {
        Registers {
            r: [
                it.next().unwrap_or(0),
                it.next().unwrap_or(0),
                it.next().unwrap_or(0),
                it.next().unwrap_or(0),
                it.next().unwrap_or(0),
                it.next().unwrap_or(0),
            ],
        }
    }
}

impl ops::Index<u32> for Registers {
    type Output = u32;
    fn index(&self, idx: u32) -> &u32 {
        match self.r.get(idx as usize) {
            Some(v) => v,
            _ => panic!("idx {} is not a register"),
        }
    }
}

impl ops::IndexMut<u32> for Registers {
    fn index_mut(&mut self, idx: u32) -> &mut u32 {
        match self.r.get_mut(idx as usize) {
            Some(v) => v,
            _ => panic!("idx {} is not a register"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RawOp {
    pub code: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    op: Op,
    inputs: RawOp,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let RawOp { a, b, c, .. } = self.inputs;
        match self.inputs.code {
            0 => write!(f, "r[{}] = r[{}] + {}", c, a, b)?,
            1 => write!(f, "r[{}] = r[{}] + r[{}]", c, a, b)?,
            2 => write!(f, "r[{}] = {} == r[{}]", c, a, b)?,
            3 => write!(f, "r[{}] = r[{}] == {}", c, a, b)?,
            4 => write!(f, "r[{}] = r[{}] == r[{}]", c, a, b)?,
            5 => write!(f, "r[{}] = {} > r[{}]", c, a, b)?,
            6 => write!(f, "r[{}] = r[{}] > {}", c, a, b)?,
            7 => write!(f, "r[{}] = r[{}] > r[{}]", c, a, b)?,
            8 => write!(f, "r[{}] = r[{}] * {}", c, a, b)?,
            9 => write!(f, "r[{}] = r[{}] * r[{}]", c, a, b)?,
            10 => write!(f, "r[{}] = {}", c, a)?,
            11 => write!(f, "r[{}] = r[{}]", c, a)?,
            12 => write!(f, "r[{}] = r[{}] & {}", c, a, b)?,
            13 => write!(f, "r[{}] = r[{}] | {}", c, a, b)?,
            _ => unimplemented!("unhandled code: {:?}", self.inputs.code),
        }
        Ok(())
    }
}

pub type Op = fn(RawOp, &mut Registers);

#[derive(Clone, Default)]
pub struct Device {
    pub ip: usize,
    pub ip_reg: u32,
    pub instructions: Vec<Instruction>,
}

impl Device {
    pub fn step(&mut self, registers: &mut Registers) -> bool {
        if let Some(instr) = self.instructions.get(self.ip) {
            registers[self.ip_reg] = self.ip as u32;
            (instr.op)(instr.inputs, registers);
            self.ip = registers[self.ip_reg] as usize;
            self.ip += 1;
            true
        } else {
            false
        }
    }

    pub fn load(input: &str) -> Device {
        let mut ip_reg = None;
        let mut instructions = Vec::new();
        for line in input.trim().lines() {
            if line.starts_with("#ip ") {
                ip_reg = Some(line[4..].parse::<u32>().unwrap());
                continue;
            }

            let (code, op) = match &line[..4] {
                "addi" => (0, op_codes::addi as Op),
                "addr" => (1, op_codes::addr as Op),
                "eqir" => (2, op_codes::eqir as Op),
                "eqri" => (3, op_codes::eqri as Op),
                "eqrr" => (4, op_codes::eqrr as Op),
                "gtir" => (5, op_codes::gtir as Op),
                "gtri" => (6, op_codes::gtri as Op),
                "gtrr" => (7, op_codes::gtrr as Op),
                "muli" => (8, op_codes::muli as Op),
                "mulr" => (9, op_codes::mulr as Op),
                "seti" => (10, op_codes::seti as Op),
                "setr" => (11, op_codes::setr as Op),
                "bani" => (12, op_codes::bani as Op),
                "bori" => (13, op_codes::bori as Op),
                other => unreachable!("unexpected instruction: {:?}", other),
            };

            let mut it = line[4..]
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u32>().unwrap());

            let a = it.next().unwrap();
            let b = it.next().unwrap();
            let c = it.next().unwrap();

            let raw = RawOp { code, a, b, c };

            instructions.push(Instruction { inputs: raw, op });
        }
        Device {
            ip: 0,
            ip_reg: ip_reg.expect("did not find #ip"),
            instructions,
        }
    }
}
