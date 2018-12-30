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

pub type Op = fn(RawOp, &mut Registers);

pub struct Device {
    pub ip: usize,
    pub ip_reg: u32,
    pub instructions: Vec<(RawOp, Op)>,
}

impl Device {
    pub fn step(&mut self, registers: &mut Registers) -> bool {
        if let Some(&(raw, op)) = self.instructions.get(self.ip) {
            registers[self.ip_reg] = self.ip as u32;
            op(raw, registers);
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

            let op = match &line[..4] {
                "addi" => op_codes::addi,
                "addr" => op_codes::addr,
                "eqir" => op_codes::eqir,
                "eqri" => op_codes::eqri,
                "eqrr" => op_codes::eqrr,
                "gtir" => op_codes::gtir,
                "gtri" => op_codes::gtri,
                "gtrr" => op_codes::gtrr,
                "muli" => op_codes::muli,
                "mulr" => op_codes::mulr,
                "seti" => op_codes::seti,
                "setr" => op_codes::setr,
                other => unreachable!("unexpected instruction: {:?}", other),
            };

            let mut it = line[4..]
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u32>().unwrap());

            let a = it.next().unwrap();
            let b = it.next().unwrap();
            let c = it.next().unwrap();

            let raw = RawOp {
                code: 0, // irrelevant
                a,
                b,
                c,
            };

            instructions.push((raw, op));
        }
        Device {
            ip: 0,
            ip_reg: ip_reg.unwrap(),
            instructions,
        }
    }
}
