use aoc_macro::{generator, solution, sol_test};

#[generator]
fn generator(v: u32) -> u32 { v }

fn power(x: u32, y: u32, serial: u32) -> i32 {
    let rack = x + 10;
    let mut power = rack * y;
    power += serial;
    power *= rack;
    power = (power % 1000) / 100;
    power as i32 - 5
}

fn power_square(size: u32, serial: u32) -> (i32, (u32, u32)) {
    let mut max = 0;
    let mut at = (0, 0);
    for x in 1..=(300 - size) {
        for y in 1..=(300 - size) {
            let mut sum = 0;
            for dx in 0..=(size - 1) {
                for dy in 0..=(size - 1) {
                    sum += power(x + dx, y + dy, serial);
                }
            }
            if max < sum {
                at = (x, y);
                max = sum;
            }
        }
    }
    (max, at)
}

#[solution(part1,
    example_input = 18,
    example = (33, 45),
    expect = (33,54))]
fn part1(serial: u32) -> (u32, u32) {
    power_square(3, serial).1
}

#[solution(part2,
    example_input = 18,
    example = (90, 269, 16),
    expect = (232, 289, 8))]
fn part2(serial: u32) -> (u32, u32, u32) {
    let mut max = 0;
    let mut at = (0, 0, 0);
    for square in 1..=300 {
        let (nmax, (x, y)) = power_square(square, serial);
        if nmax > max {
            max = nmax;
            at = (x, y, square);
        }
    }
    at
}

sol_test!(p1: power(3, 5, 8), 4);
sol_test!(p2: power(122, 79, 57), -5);
sol_test!(p3: power(217, 196, 39), 0);
sol_test!(p4: power(101, 153, 71), 4);

sol_test!(p5: part1(42), (21, 61));
sol_test!(p6: part2(42), (232, 251, 12));

static INPUT: u32 = 5235;
