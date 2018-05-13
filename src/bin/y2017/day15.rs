#![feature(stdsimd)]

#[macro_use]
extern crate advent_of_code;

use std::simd::u64x4;

// See https://ariya.blogspot.com/2007/02/modulus-with-mersenne-prime.html
// (num * factor) % 2147483648
fn generate(mut num: u64, factor: u64) -> u64 {
    num *= factor;
    const LOW_BITS: u64 = (1u64 << 31) - 1;
    num = (num >> 31) + (num & LOW_BITS);
    num = ((num >> 31) + num) & LOW_BITS;
    num
}

fn generate4(mut num: u64x4, factor: u64) -> u64x4 {
    num *= factor;
    const LOW_BITS: u64 = (1u64 << 31) - 1;
    num = (num >> 31) + (num & LOW_BITS);
    num = ((num >> 31) + num) & LOW_BITS;
    num
}

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

// These are the result of applying generate 4 times, in effect:
// g(g(g(g(1, factor), factor), factor), factor)
const A_FACTOR_4: u64 = 984943658;
const B_FACTOR_4: u64 = 1914720637;

pub fn part1((a, b): (u64, u64)) -> u16 {
    let mask = 0xffff;
    let mut count = 0;
    let (mut a, mut b) = prepare_from_input(a, b);
    for _ in 0..10_000_000 {
        let c = (a & mask).eq(b & mask);
        for i in 0..4 {
            count += c.extract(i) as u16;
        }
        a = generate4(a, A_FACTOR_4);
        b = generate4(b, B_FACTOR_4);
    }
    count
}

struct Generator {
    state: u64x4,
    queue: Vec<u64>,
    mask: u64,
    factor_4: u64,
    first: bool,
}

impl Generator {
    fn new_a(initial: u64) -> Generator {
        Generator::new(initial, 3, A_FACTOR, A_FACTOR_4)
    }

    fn new_b(initial: u64) -> Generator {
        Generator::new(initial, 7, B_FACTOR, B_FACTOR_4)
    }

    fn new(initial: u64, mask: u64, factor: u64, factor_4: u64) -> Generator {
        let mut state = u64x4::new(initial, 0, 0, 0);
        for i in 1..4 {
            state = state.replace(i, generate(state.extract(i - 1), factor));
        }
        Generator {
            state,
            mask,
            queue: Vec::new(),
            factor_4,
            first: true,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        while self.queue.len() < 2 {
            let start = if self.first { 1 } else { 0 };
            for i in start..4 {
                if self.state.extract(i) & self.mask == 0 {
                    self.queue.push(self.state.extract(i));
                }
            }
            self.first = false;
            self.state = generate4(self.state, self.factor_4);
        }
        Some(self.queue.remove(0))
    }
}

pub fn part2((a, b): (u64, u64)) -> usize {
    let mut count = 0;
    let mut a_gen = Generator::new_a(a);
    let mut b_gen = Generator::new_b(b);
    for _ in 0..5_000_000 {
        let a = a_gen.next().unwrap();
        let b = b_gen.next().unwrap();
        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }
    count
}

fn prepare_from_input(a: u64, b: u64) -> (u64x4, u64x4) {
    let mut a = u64x4::new(a, 0, 0, 0);
    let mut b = u64x4::new(b, 0, 0, 0);
    for i in 1..4 {
        a = a.replace(i, generate(a.extract(i - 1), A_FACTOR));
        b = b.replace(i, generate(b.extract(i - 1), B_FACTOR));
    }
    (a, b)
}

#[test]
fn part1_1() {
    assert_eq!(part1((65, 8921)), 588);
}

#[test]
fn part2_1() {
    assert_eq!(part2((65, 8921)), 309);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 569);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 298);
}

gen_single!();

pub static INPUT: (u64, u64) = (116, 299);
