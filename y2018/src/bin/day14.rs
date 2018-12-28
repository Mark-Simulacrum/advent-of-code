#![feature(try_from)]
use aoc_macro::{generator, solution, sol_test};
use std::convert::TryInto;

aoc_macro::day!();

#[generator]
fn generator<T>(input: T) -> T {
    input
}

fn digits(mut v: usize) -> Vec<u8> {
    let mut o = Vec::new();
    loop {
        o.push((v % 10) as u8);
        v /= 10;
        if v == 0 {
            break;
        }
    }
    o.reverse();
    o
}

fn recombine(v: &[usize]) -> usize {
    v.iter().rev().cloned().enumerate().map(|(place, value)| {
        10usize.pow(place as u32) * value
    }).sum()
}

sol_test!(digits_1: digits(3232), [3, 2, 3, 2]);
sol_test!(digits_2: digits(234324), [2, 3, 4, 3, 2, 4]);
sol_test!(recombine_1: recombine(&[0, 1, 2, 3]), 123);
sol_test!(recombine_2: recombine(&[5, 6, 2, 3]), 5623);

#[solution(part1,
    example_input = &[9],
    example = [5, 1, 5, 8, 9, 1, 6, 7, 7, 9],
    expect = [6, 1, 0, 7, 1, 0, 1, 5, 4, 4])]
fn part1(count: &[usize]) -> [u8; 10] {
    let count = recombine(&count);
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut a = 0;
    let mut b = 1;
    while recipes.len() <= (count + 20) {
        let sum = recipes[a] as usize + recipes[b] as usize;
        let digits = digits(sum);
        recipes.extend(digits);
        a += recipes[a] as usize + 1;
        a %= recipes.len();
        b += recipes[b]  as usize+ 1;
        b %= recipes.len();
    }
    recipes[count..count + 10].try_into().unwrap()
}

sol_test!(part1_1: part1(&[5]), [0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
sol_test!(part1_2: part1(&[1, 8]), [9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
sol_test!(part1_3: part1(&[2, 0, 1, 8]), [5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);

#[solution(part2,
    example_input = &[5, 1, 5, 8, 9],
    example = 9,
    expect = 20291131)]
fn part2(input: &[usize]) -> usize {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut a = 0;
    let mut b = 1;
    let input = input.iter().map(|i| *i as u8).collect::<Vec<_>>();
    let mut last_check = 0;
    loop {
        let sum = recipes[a] as usize + recipes[b] as usize;
        let digits = digits(sum);
        recipes.extend(digits);
        a += recipes[a] as usize + 1;
        a %= recipes.len();
        b += recipes[b] as usize + 1;
        b %= recipes.len();
        if recipes.len() % 1000 == 0 {
            for (idx, window) in recipes.windows(input.len()).enumerate().skip(last_check) {
                if window == &input[..] {
                    return idx;
                }
            }
            last_check = recipes.len();
        }
    }
}

sol_test!(part2_1: part2(&[0, 1, 2, 4, 5]), 5);
sol_test!(part2_2: part2(&[9, 2, 5, 1, 0]), 18);
sol_test!(part2_3: part2(&[5, 9, 4, 1, 4]), 2018);
sol_test!(part2_4: part2(&[5, 1, 5, 8, 9, 1, 6, 7, 7, 9]), 9);

static INPUT: &[usize] = &[1, 1, 0, 2, 0, 1];
