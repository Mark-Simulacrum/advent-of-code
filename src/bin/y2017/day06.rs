use std::collections::{HashSet, HashMap};

fn max(list: &[u32]) -> (usize, u32) {
    list.iter().cloned().enumerate()
        // If the maximum values are tied, we want the lowest index to win.
        // Since this is a max, the largest negative index will be the smallest index.
        .max_by_key(|x| (x.1, -(x.0 as isize)))
        .unwrap()
}

fn redistribute(blocks: &mut [u32]) {
    let (max_pos, mut max) = max(&*blocks);
    blocks[max_pos] = 0;
    for i in (0..blocks.len()).into_iter().cycle().skip(max_pos + 1) {
        if max == 0 {
            break;
        }
        blocks[i] += 1;
        max -= 1;
    }
}

pub fn part1(s: &[u32]) -> usize {
    let mut blocks = Vec::from(s);
    let mut seen = HashSet::new();
    while seen.insert(blocks.clone()) {
        redistribute(&mut blocks);
    }
    seen.len()
}

// How long ago did we see the looped state?
pub fn part2(s: &[u32]) -> usize {
    let mut seen = HashMap::new();
    let mut blocks = Vec::from(s);
    loop {
        let len = seen.len();
        if let Some(last) = seen.insert(blocks.clone(), len) {
            return len - last;
        }
        redistribute(&mut blocks);
    }
}

#[test]
fn part1_1() {
    assert_eq!(part1(&[0, 2, 7, 0]), 5);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 6681);
}

#[test]
fn part2_1() {
    assert_eq!(part2(&[0, 2, 7, 0]), 4);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 2392);
}


pub static INPUT: &[u32] = &[4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
