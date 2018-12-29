use super::{Registers, RawOp};

pub fn addr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] + regs[raw.b];
}

pub fn addi(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] + raw.b;
}

pub fn mulr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] * regs[raw.b];
}

pub fn muli(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] * raw.b;
}

pub fn banr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] & regs[raw.b];
}

pub fn bani(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] & raw.b;
}

pub fn borr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] | regs[raw.b];
}

pub fn bori(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a] | raw.b;
}

pub fn setr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = regs[raw.a];
}

pub fn seti(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = raw.a;
}

pub fn gtir(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = (raw.a > regs[raw.b]) as u32;
}

pub fn gtri(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = (regs[raw.a] > raw.b) as u32;
}

pub fn gtrr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = (regs[raw.a] > regs[raw.b]) as u32;
}

pub fn eqir(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = (raw.a == regs[raw.b]) as u32;
}

pub fn eqri(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = (regs[raw.a] == raw.b) as u32;
}

pub fn eqrr(raw: RawOp, regs: &mut Registers) {
    regs[raw.c] = (regs[raw.a] == regs[raw.b]) as u32;
}
