use std::collections::VecDeque;
use std::fmt;
use std::io::Write;

use md5;
use smallvec::SmallVec;

use advent_of_code::HashIter;

fn to_hex<B: fmt::LowerHex>(s: B) -> [u8; 32] {
    let mut v = [0; 32];
    write!(&mut v[..], "{:x}", s).unwrap();
    v
}

fn md5<B: AsRef<[u8]>>(s: B) -> [u8; 32] {
    to_hex(&md5::compute(s))
}

struct Hash {
    triple: Option<u8>,
    five: SmallVec<[u8; 1]>,
}

impl Hash {
    fn from_bytes(s: [u8; 32]) -> Hash {
        let triple_byte = s.windows(3)
            .find(|w| w[0] == w[1] && w[1] == w[2])
            .map(|w| w[0]);
        let mut five_bytes = SmallVec::new();
        for window in s.windows(5) {
            if window.iter().all(|x| *x == window[0]) {
                five_bytes.push(window[0]);
            }
        }
        Hash {
            triple: triple_byte,
            five: five_bytes,
        }
    }
}

fn compute(s: &str, stretch: bool) -> usize {
    let mut keys_found = 0;
    let mut window = VecDeque::with_capacity(1001);
    let mut hashes = HashIter::new(s)
        .map(|bytes| {
            if stretch {
                let mut bytes = to_hex(bytes);
                for _ in 0..2016 {
                    bytes = md5(&bytes);
                }
                bytes
            } else {
                to_hex(bytes)
            }
        })
        .map(|bytes| Hash::from_bytes(bytes));
    window.extend(hashes.by_ref().take(1001));
    for idx in 0.. {
        if let Some(byte) = window[0].triple {
            for hash in window.iter().skip(1) {
                if hash.five.contains(&byte) {
                    keys_found += 1;
                    if keys_found == 64 {
                        return idx;
                    }
                    break;
                }
            }
        }
        let _ = window.pop_front();
        window.push_back(hashes.next().unwrap());
    }
    unreachable!()
}

pub fn part1(s: &str) -> usize {
    compute(s, false)
}

pub fn part2(s: &str) -> usize {
    compute(s, true)
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 35186);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 22429);
}

pub const INPUT: &str = "jlmsuwbz";
