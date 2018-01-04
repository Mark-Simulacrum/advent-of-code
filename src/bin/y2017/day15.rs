fn generator(num: u64, factor: u64) -> u64 {
    (num * factor) % 2_147_483_647
}

fn generator_mod(mut num: u64, factor: u64, modulo: u64) -> u64 {
    num = generator(num, factor);
    while num % modulo != 0 {
        num = generator(num, factor);
    }
    num
}

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

pub fn part1((mut a, mut b): (u64, u64)) -> u16 {
    let mask = 0xffff;
    let mut count = 0;
    for _ in 0..40_000_000 {
        a = generator(a, A_FACTOR);
        b = generator(b, B_FACTOR);
        if (a & mask) == (b & mask) {
            count += 1;
        }
    }
    count
}

pub fn part2((mut a, mut b): (u64, u64)) -> usize {
    let mask = 0b1111_1111_1111_1111;
    let mut count = 0;
    for _ in 0..5_000_000 {
        a = generator_mod(a, A_FACTOR, 4);
        b = generator_mod(b, B_FACTOR, 8);
        if (a & mask) == (b & mask) {
            count += 1;
        }
    }
    count
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

pub static INPUT: (u64, u64) = (116, 299);
