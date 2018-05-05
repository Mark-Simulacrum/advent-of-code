// Derived from reading about the Josephus problem [https://en.wikipedia.org/wiki/Josephus_problem]
// and k=2.
pub fn part1(n: usize) -> usize {
    let m = 0usize.leading_zeros() - n.leading_zeros();
    2 * (n - 2usize.pow(m - 1)) + 1
}

// Derived from analysis of the data output; it's a function that sawtooths relatively predictably
pub fn part2(n: usize) -> usize {
    let greatest_pow_3 = (0..).take_while(|i| 3usize.pow(*i) <= n).last().unwrap();
    let m = 3usize.pow(greatest_pow_3);
    // all powers of 3 match perfectly
    if n == m {
        n
    // for every power, 3^power (m) away from it is a range of +1
    // That is, after `9`, for 10..=18 the sequence is 1..=9
    //          after `27`, for 28..=54 the sequence is 1..=27
    } else if (n - m) <= m {
        n - m
    // And after we've exhausted that range, we go by 2s. This really means that we're now m away
    // from the power. From here on out, we want to increment by 2 for every n -- so,
    // 2 * n. By 2*n we're going into the next m's range, so we step back a power of 3 to get to
    // where we want to be.
    } else {
        2 * n - 3usize.pow(greatest_pow_3 + 1)
    }
}

#[test]
fn part1_1() {
    assert_eq!(part1(5), 3);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 1834471);
}

#[test]
fn part2_1() {
    assert_eq!(part2(5), 2);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 1420064);
}

pub static INPUT: usize = 3014387;
