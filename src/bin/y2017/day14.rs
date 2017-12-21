use day10::knot_hash;
use std::u128;

pub fn part1(s: &str) -> u32 {
    parse_rows(s).iter().map(|r| r.count_ones()).sum()
}

fn wipe_region(rows: &mut [u128], row: usize, col: u8) -> u32 {
    if row == rows.len() { return 0; }
    if col > 127 { return 0; }
    if rows[row] & (1u128 << col) == 0 { return 0; }
    rows[row] &= !(1u128 << col);
    wipe_region(rows, row, col.saturating_sub(1));
    wipe_region(rows, row, col.saturating_add(1));
    wipe_region(rows, row.saturating_add(1), col);
    wipe_region(rows, row.saturating_sub(1), col);
    1
}

fn remove_groups(rows: &mut [u128]) -> u32 {
    let mut groups = 0;
    for i in 0..rows.len() {
        for j in 0..128 {
            groups += wipe_region(rows, i, j);
        }
    }
    groups
}

#[test]
fn remove_groups_1() {
    assert_eq!(remove_groups(&mut vec![
        0b01010,
        0b01010,
        0b01110,
    ]), 1);
}

#[test]
fn remove_groups_2() {
    assert_eq!(remove_groups(&mut vec![
        0b01001111001110111,
        0b01011001001010101,
        0b01110001001010101,
        0b00000111111011101
    ]), 1);
}

#[test]
fn remove_groups_3() {
    assert_eq!(remove_groups(&mut vec![
        0b10100000000000000,
        0b01100000000000000,
        0b00000000000000000,
        0b00000000000000000
    ]), 2);
}

fn parse_rows(s: &str) -> [u128; 128] {
    let mut rows = [0; 128];
    for i in 0..128 {
        let input = format!("{}-{}", s, i);
        let decimal = knot_hash(&input);
        rows[i as usize] = decimal;
    }
    rows
}

pub fn part2(s: &str) -> u32 {
    let mut rows = parse_rows(s);
    remove_groups(&mut rows)
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 8108);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 8214);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE), 1242);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 1093);
}

#[cfg(test)]
pub static EXAMPLE: &str = "flqrgnkx";
pub static INPUT: &str = "hxtvlmkl";
