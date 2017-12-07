fn max(list: &[u32]) -> (usize, u32) {
    let mut max = 0;
    let mut max_idx = 0;
    let mut idx = list.len() - 1;
    loop {
        let i = list[idx];
        if i >= max {
            max_idx = idx;
            max = i;
        }
        if idx == 0 { break; }
        idx -= 1;
    }
    (max_idx, max)
}

// step
fn f(blocks: &mut [u32]) -> &mut [u32] {
    let (max_pos, mut max) = max(&*blocks);
    blocks[max_pos] = 0;
    let mut j = 0;
    let len = blocks.len();
    while max > 0 {
        blocks[(max_pos + j + 1) % len] += 1;
        max -= 1;
        j += 1;
    }
    blocks
}

fn exec(s: &[u32]) -> (usize, usize) {
    let x0 = Vec::from(s);

    let mut tortoise_0 = x0.clone();
    let mut hare_0 = x0.clone();

    let tortoise = f(&mut tortoise_0);
    let hare = f(f(&mut hare_0));
    while tortoise != hare {
        f(tortoise);
        f(f(hare));
    }

    let mut mu = 0;
    let mut tortoise_0 = x0.clone();
    let tortoise = &mut tortoise_0[..];
    while tortoise != hare {
        f(tortoise);
        f(hare);
        mu += 1;
    }

    let mut lam = 1;
    let mut hare_0 = Vec::from(&*tortoise);
    let hare = f(&mut hare_0);
    while tortoise != hare {
        f(hare);
        lam += 1;
    }

    (lam, mu)
}

pub fn part1(s: &[u32]) -> usize {
    let (a, b) = exec(s);
    a + b
}

// How long ago did we see the looped state?
pub fn part2(s: &[u32]) -> usize {
    let (a, _) = exec(s);
    a
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
