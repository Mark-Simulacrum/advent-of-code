use advent_of_code::VecMap;
#[allow(unused)]
use itertools::Itertools;
use std::fmt;
use std::iter;

#[derive(Copy, Clone, Debug)]
struct Layer {
    range: u8,
}

#[derive(Clone)]
struct Layers {
    scanner: u64,
    layers: Vec<Layer>,
}

impl Layers {
    fn parse(s: &str) -> Layers {
        let mut layers = Vec::new();
        for l in s.trim().lines() {
            let mut items = l.split(':');
            let depth = items.next().unwrap().parse::<u8>().unwrap();
            let range = items.next().unwrap()[1..].parse::<u8>().unwrap();
            let l = layers.len();
            layers.extend(iter::repeat(Layer { range: 0 }).take(depth as usize - l));
            layers.push(Layer {
                range: range.saturating_sub(1),
            });
        }
        Layers { layers: layers, scanner: 0 }
    }

    fn run(&mut self) -> usize {
        let mut severity = 0;
        for (depth, layer) in self.layers.iter().enumerate() {
            // multiply range by 2 for trip there and back again
            if layer.range != 0 && self.scanner % (layer.range as u64 * 2) == 0 {
                severity += depth * (layer.range as usize + 1);
            }
            self.scanner += 1;
        }
        severity
    }

}

pub fn part1(s: &str) -> usize {
    Layers::parse(s).run()
}

fn modulo(a: i128, b: i128) -> u64 {
    let mut ret = a % b;
    if ret < 0 {
        ret += b;
    }
    ret as u64
}

// Algorithm taken from
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode.
#[allow(unused)]

#[derive(Copy, Clone)]
struct ExtendedGcd {
    gcd: i128,
    m: i128,
    n: i128,
}

impl ExtendedGcd {
    fn new(a: i128, b: i128) -> ExtendedGcd {
        let mut s = 0;
        let mut old_s = 1;
        let mut t = 1;
        let mut old_t = 0;
        let mut r = b;
        let mut old_r = a;
        while r != 0 {
            let q = old_r / r;
            let r1 = r;
            let s1 = s;
            let t1 = t;
            r = old_r - q * r;
            s = old_s - q * s;
            t = old_t - q * t;
            old_r = r1;
            old_s = s1;
            old_t = t1;
        }
        ExtendedGcd {
            gcd: old_r,
            m: old_s,
            n: old_t,
        }
    }
}

#[allow(unused)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut d = 0;
    while a % 2 == 0 && b % 2 == 0 {
        a /= 2;
        b /= 2;
        d += 1;
    }
    while a != b {
        if a % 2 == 0 {
            a /= 2;
        } else if b % 2 == 0 {
            b /= 2;
        } else if a > b {
            a = (a - b) / 2;
        } else {
            b = (b - a) / 2;
        }
    }
    a * 2u64.pow(d)
}

// x % range != 0
// a ≡ b mod n
// => a mod n == b mod n
//
// Normally, we have a set of congruence relations
// (delay + depth) ≡ 0 (mod range), but we can simplify:
// (delay + depth) ≡ 0 (mod range)
// delay ≡ -depth (mod range)
// delay (mod range) = -depth (mod range)
//
// This reduces the number of modulos we need to take for a given delay,
// and as such, enhances runtime for relatively little bookkeeping cost.
//
// Since we have a system of inequalities, and the Chinese Remainder Theorem
// applies to equalities, we invert the inequality, instead generating a set
// of integers which delay % range should equal.
//
// range: [possible modulos]
// 2: [0]
// 4: [1, 2, 3]
//
// x = 0 mod 2
// and
// x = 1 mod 4 OR
// x = 2 mod 4 OR
// x = 3 mod 4
//
// This means we have 3 different systems to solve.

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Congruence {
    a: i128,
    n: i128,
}

impl fmt::Debug for Congruence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mod {}", self.a, self.n)
    }
}

impl Congruence {
    // See https://math.stackexchange.com/questions/911902/
    fn combine(self, other: Congruence) -> Option<Congruence> {
        let ExtendedGcd { gcd, m, n } = ExtendedGcd::new(self.n, other.n);
        if modulo(self.a, gcd) == modulo(other.a, gcd) {
            let lcm = (self.n*other.n) / gcd;
            let ret = Congruence {
                n: lcm,
                a: (other.a * m * self.n + self.a * n * other.n) / gcd,
            }.normalize();
            Some(ret)
        } else {
            None
        }
    }

    fn normalize(self) -> Congruence {
        let mut a = self.a;
        a -= (a / self.n) * self.n;
        if a < 0 {
            a += (a.abs() / self.n + 1) * self.n;
        }
        Congruence { n: self.n, a }
    }
}

pub fn part2(s: &str) -> u64 {
    let layers = Layers::parse(s);
    let ranges = layers.layers.iter()
        .enumerate()
        .filter(|&(_, l)| l.range != 0)
        .map(|(i, l)| (i as u64, (l.range as u64) * 2))
        .map(|(d, r)| Congruence { n: r as i128, a: modulo(-(d as i128), r as i128) as i128 })
        .collect::<Vec<_>>();
    let mut ranges_map = VecMap::with_capacity(ranges.len());
    for &Congruence { n: range, a: x } in &ranges {
        ranges_map.get_or_insert_with(range, Vec::new).push(x as u8);
    }
    let mut system = ranges_map.into_iter().map(|(range, xs)| {
        (0..range as u8).into_iter().filter(|n| !xs.contains(n))
            .map(|x| Congruence { n: range, a: x as i128 }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    system.sort_by_key(|c| c[0].n);
    while system.len() >= 2 {
        let mut solutions = Vec::with_capacity(system[0].len() * system[1].len());
        for a in &system[0] {
            for b in &system[1] {
                if let Some(combined) = a.combine(*b) {
                    solutions.push(combined);
                }
            }
        }
        assert!(!solutions.is_empty(), "{:?} and {:?} has no solutions", system[0], system[1]);
        solutions.sort_unstable();
        solutions.dedup();
        system[0] = solutions;
        system.remove(1);
    }
    system[0].iter().min_by_key(|x| x.a).unwrap().a as u64
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 24);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 1900);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE), 10);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 3966414);
}


#[cfg(test)]
static EXAMPLE: &str = "
0: 3
1: 2
4: 4
6: 4
";

pub static INPUT: &str = "
0: 3
1: 2
2: 4
4: 4
6: 5
8: 6
10: 6
12: 6
14: 6
16: 8
18: 8
20: 8
22: 8
24: 10
26: 8
28: 8
30: 12
32: 14
34: 12
36: 10
38: 12
40: 12
42: 9
44: 12
46: 12
48: 12
50: 12
52: 14
54: 14
56: 14
58: 12
60: 14
62: 14
64: 12
66: 14
70: 14
72: 14
74: 14
76: 14
80: 18
88: 20
90: 14
98: 17
";
