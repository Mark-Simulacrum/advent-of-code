use std::ops;
use std::fmt;

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
    where I: Iterator<Item=u32>,
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
