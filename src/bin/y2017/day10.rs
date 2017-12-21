#[cfg(test)]
use quickcheck::TestResult;

pub fn part1(input: &str) -> usize {
    let input = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut list = (0..256).into_iter().collect::<Vec<usize>>();
    let mut cur = 0;
    let mut skip = 0;
    run(&mut list, &mut cur, &mut skip, &input);
    list[0] * list[1]
}

pub fn part2(input: &str) -> String {
    format!("{:x}", knot_hash(input))
}

pub fn knot_hash(input: &str) -> u128 {
    let mut input = input.as_bytes().iter().map(|x| *x as usize).collect::<Vec<usize>>();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut list = (0..256).into_iter().collect::<Vec<usize>>();
    let mut cur = 0;
    let mut skip = 0;
    for _ in 0..64 {
        run(&mut list, &mut cur, &mut skip, &input[..]);
    }
    let mut dense = 0u128;
    for i in 0..16 {
        let base = i * 16;
        let mut o = list[base];
        for m in 1..16 {
            o ^= list[base + m];
        }
        dense += (o as u128) << (i as u128 * 8);
    }
    dense.to_be()
}

#[inline(always)]
fn add(i: usize, n: usize, constraint: usize) -> usize {
    if constraint == 0 { return 0; }
    let mut tot = i + n;
    while tot >= constraint {
        tot -= constraint;
    }
    tot
}

fn reverse_after<T>(slice: &mut [T], a: usize, length: usize) {
    if a + length <= slice.len() {
        slice[a..(a + length)].reverse();
    } else {
        let mut idx = 0;
        let mut from = a;
        let mut to = length - (slice.len() - a) - 1;
        while idx < (length / 2) {
            slice.swap(from, to);
            from += 1;
            if from == slice.len() {
                from = 0;
            }
            if to == 0 {
                to = slice.len() - 1;
            } else {
                to -= 1;
            }
            idx += 1;
        }
    }
}

#[quickcheck]
fn circle_rev_any(v: Vec<usize>, a: usize, length: usize) -> TestResult {
    if a >= v.len() || length > v.len() {
        return TestResult::discard();
    }
    let mut circle = v.clone();
    reverse_after(&mut circle, a, length);
    reverse_after(&mut circle, a, length);
    TestResult::from_bool(circle == v)
}

#[test]
fn circle_rev_1() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let mut circle = v.clone();
    let l = circle.len();
    reverse_after(&mut circle, 0, l);
    v.reverse();
    assert_eq!(circle, v);
}

#[test]
fn circle_rev_2() {
    let v = vec![0, 1, 2, 3, 4, 5, 6];
    let mut circle = v.clone();
    reverse_after(&mut circle, 5, 5);
    assert_eq!(circle, &[0, 6, 5, 3, 4, 2, 1]);
}

fn run(list: &mut [usize], cur: &mut usize, skip: &mut usize, lengths: &[usize]) {
    for &len in lengths {
        reverse_after(list, *cur, len);
        *cur = add(*cur, len + *skip, list.len());
        *skip += 1;
    }
}

#[test]
fn part1_1() {
    let mut c = vec![0, 1, 2, 3, 4];
    run(&mut c, &mut 0, &mut 0, &[3, 4, 1, 5]);
    assert_eq!(c[0] * c[1], 12);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 7888);
}

#[test]
fn part2_1() {
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
}

#[test]
fn part2_2() {
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
}

#[test]
fn part2_3() {
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
}

#[test]
fn part2_4() {
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), "decdf7d377879877173b7f2fb131cf1b");
}

pub static INPUT: &str = "70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41";
