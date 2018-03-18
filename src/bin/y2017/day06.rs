use advent_of_code::cycle;

fn max(list: &[u32]) -> (usize, u32) {
    let mut max = 0;
    let mut max_idx = 0;
    let mut idx = list.len() - 1;
    loop {
        let i = unsafe { *list.get_unchecked(idx) };
        if i >= max {
            max_idx = idx;
            max = i;
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    (max_idx, max)
}

fn step(mut list: Vec<u32>) -> Vec<u32> {
    unsafe {
        let (max_pos, mut max) = max(&*list);
        *list.get_unchecked_mut(max_pos) = 0;
        let mut j = 1;
        let len = list.len();
        while max > 0 {
            *list.get_unchecked_mut((max_pos + j) % len) += 1;
            max -= 1;
            j += 1;
        }
        list
    }
}

pub fn part1(s: &[u32]) -> usize {
    let cycle = cycle::find(s.to_vec(), step);
    cycle.start() + cycle.length()
}

// How long ago did we see the looped state?
pub fn part2(s: &[u32]) -> usize {
    let cycle = cycle::find(s.to_vec(), step);
    cycle.length()
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
