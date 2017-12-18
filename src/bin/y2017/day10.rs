#[cfg(test)]
use quickcheck::TestResult;

fn add(i: usize, n: usize, constraint: usize) -> usize {
    let mut tot = i + n;
    while tot >= constraint {
        tot -= constraint;
    }
    tot
}

struct Circle(Vec<usize>);

impl Circle {
    fn reverse_after(&mut self, a: usize, length: usize) {
        let l = self.0.len();
        if a + length <= l {
            self.0[a..(a + length)].reverse();
        } else {
            let mut sublist = vec![];
            let mut i = a;
            let mut idx_a = 0;
            while sublist.len() != length {
                sublist.push(self.0[i]);
                i += 1;
                if i == self.0.len() {
                    i = 0;
                    idx_a = sublist.len();
                }
            }
            sublist.reverse();
            let mut i = a;
            for el in &sublist[0..idx_a] {
                self.0[i] = *el;
                i += 1;
            }
            let mut i = 0;
            for el in &sublist[idx_a..] {
                self.0[i] = *el;
                i += 1;
            }
        }
    }

    #[cfg_attr(not(test), allow(unused))]
    fn reverse_range(&mut self, a: usize, b: usize) {
        if b == a { return; }
        if b < a {
            let l = self.0.len();
            self.reverse_after(a, l - a + b);
        } else {
            self.reverse_after(a, b - a);
        }
    }
}

#[quickcheck]
fn circle_rev_simple(mut v: Vec<usize>, a: usize, b: usize) -> TestResult {
    if a >= v.len() || b >= v.len() || b < a {
        return TestResult::discard();
    }
    let mut circle = Circle(v.clone());
    circle.reverse_range(a, b);
    v[a..b].reverse();
    TestResult::from_bool(&*circle.0 == &v[..])
}

#[quickcheck]
fn circle_rev_any(v: Vec<usize>, a: usize, b: usize) -> TestResult {
    if a >= v.len() || b >= a + v.len() - 1 {
        return TestResult::discard();
    }
    let mut circle = Circle(v.clone());
    circle.reverse_range(a, b);
    circle.reverse_range(a, b);
    TestResult::from_bool(&*circle.0 == &v[..])
}

pub fn part1(input: &str) -> usize {
    let input = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut list = Circle((0..256).into_iter().collect::<Vec<usize>>());
    let mut cur = 0;
    let mut skip = 0;
    run(&mut list, &mut cur, &mut skip, &input);
    list.0[0] * list.0[1]
}

fn run(list: &mut Circle, cur: &mut usize, skip: &mut usize, lengths: &[usize]) {
    for &len in lengths {
        list.reverse_after(*cur, len);
        *cur = add(*cur, len + *skip, list.0.len());
        *skip += 1;
    }
}

#[test]
fn part1_1() {
    let mut c = Circle(vec![0, 1, 2, 3, 4]);
    run(&mut c, &mut 0, &mut 0, &[3, 4, 1, 5]);
    assert_eq!(c.0[0] * c.0[1], 12);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 7888);
}

pub fn part2(input: &str) -> String {
    let mut input = input.as_bytes().iter().map(|x| *x as usize).collect::<Vec<usize>>();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut list = Circle((0..256).into_iter().collect::<Vec<usize>>());
    let mut cur = 0;
    let mut skip = 0;
    for _ in 0..64 {
        run(&mut list, &mut cur, &mut skip, &input[..]);
    }
    let mut dense = String::new();
    for i in 0..16 {
        let base = i * 16;
        let mut o = list.0[base];
        for m in 1..16 {
            o ^= list.0[base + m];
        }
        dense.push_str(&format!("{:02x}", o));
    }
    dense
}

#[test]
fn part2_1() {
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
}

#[test]
fn part2_2() {
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
}

#[test]
fn part2_3() {
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
}

#[test]
fn part2_4() {
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), "decdf7d377879877173b7f2fb131cf1b");
}

pub static INPUT: &str = "70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41";
