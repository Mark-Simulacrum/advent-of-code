fn compute(input: &[u8], disk_size: usize) -> String {
    let mut data = Vec::from(input);
    while data.len() < disk_size {
        let a = data;
        let mut b = a.clone();
        b.reverse();
        for x in &mut b {
            *x = if *x == 1 { 0 } else { 1 };
        }
        data = a;
        data.push(0);
        data.extend(b);
    }

    data.truncate(disk_size);

    // The problem specifies that we take chunks of size 2, and patterns 11,00
    // -> 1, 01,10 -> 0.  This is a XNOR operation. XNOR recursively repeated
    // over a range of data returns 1 for inputs where the number of set bits
    // is even; we can apply that knowledge by examining chunks of size equal
    // to the largest multiple of 2 less than the data length (which is N &
    // !(N-1) -- unsetting all bits less than the highest set bit).
    data.chunks(data.len() & !(data.len() - 1))
        .map(|chunk| {
            if chunk.iter().filter(|x| **x == 1).count() % 2 == 0 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>()
}

pub fn part1(input: &[u8]) -> String {
    compute(input, 272)
}

pub fn part2(input: &[u8]) -> String {
    compute(input, 35651584)
}

#[test]
fn part1_1() {
    assert_eq!(part1(INPUT), "10101001010100001");
}

#[test]
fn part2_1() {
    assert_eq!(part2(INPUT), "10100001110101001");
}

pub const INPUT: &[u8] = &[1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1];
