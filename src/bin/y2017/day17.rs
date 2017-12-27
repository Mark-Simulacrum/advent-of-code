pub fn part1(step: usize) -> u16 {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut pos = 0;
    for i in 1..2018 {
        let insert = (pos + step) % buf.len();
        buf.insert(insert + 1, i);
        pos = insert + 1;
    }
    let next = (buf.iter().position(|v| *v == 2017).unwrap() + 1) % buf.len();
    buf[next]
}

pub fn part2(step: usize) -> u32 {
    let step = step as u32;
    let mut zero_position = 0;
    let mut last_next_value = 0;
    let mut pos = 0;
    for i in 1..50_000_001 {
        let insert = ((pos + step) % i) + 1;
        if insert == zero_position {
            zero_position += 1;
        }
        if insert == zero_position + 1 {
            last_next_value = i;
        }
        pos = insert;
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
