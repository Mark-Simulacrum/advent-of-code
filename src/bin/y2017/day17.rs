pub fn part1(step: usize) -> usize {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut pos = 0;
    for i in 1..2018 {
        let insert = ((pos + step) % i) + 1;
        buf.insert(insert, i as u16);
        pos = insert;
    }
    buf[(pos + 1) % buf.len()] as usize
}

pub fn part2(step: usize) -> usize {
    let mut last_next_value = 0;
    let mut pos = 0;
    for i in 1..50_000_001 {
        let insert = (pos + step) % i;
        if insert == 0 {
            last_next_value = i;
        }
        pos = insert + 1;
    }
    last_next_value
}

#[test]
fn part1_1() {
    assert_eq!(part1(3), 638);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 1173);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 1930815);
}

pub const INPUT: usize = 304;
