fn parse(s: &str) -> Vec<(u32, u32)> {
    s.trim()
        .lines()
        .map(|v| {
            let mut i = v.split('-').map(|x| x.parse::<u32>().unwrap());
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect()
}

pub fn part1(s: &str) -> u32 {
    let blacklist = parse(s);
    let mut i = 0u32;
    loop {
        let mut valid = true;
        for &(a, b) in &blacklist {
            if a <= i && i <= b {
                valid = false;
                i = b;
                break;
            }
        }
        if valid {
            return i;
        }
        i += 1;
    }
}

pub fn part2(s: &str) -> usize {
    let blacklist = parse(s);
    let mut allowed = 0;
    let mut i = 0u32;
    loop {
        let mut valid = true;
        for &(a, b) in &blacklist {
            if a <= i && i <= b {
                valid = false;
                i = b;
                break;
            }
        }
        if valid {
            allowed += 1;
        }
        if i == u32::max_value() {
            break;
        }
        i += 1;
    }
    allowed
}

#[test]
fn part1_1() {
    assert_eq!(part1(INPUT), 17348574);
}

#[test]
fn part2_1() {
    assert_eq!(part2(INPUT), 120);
}

pub static INPUT: &str = include_str!("day20.input");
