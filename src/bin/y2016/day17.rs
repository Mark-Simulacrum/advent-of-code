use md5;
use smallvec::SmallVec;
use std::cmp::{max, min, Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;

use advent_of_code::{hi_nib, lo_nib};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position(u8, u8, String);

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.2.len().partial_cmp(&other.2.len())
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn is_open(nib: u8) -> bool {
    nib == 0xf || nib == 0xb || nib == 0xc || nib == 0xd || nib == 0xe
}

impl Position {
    fn move_down(&self) -> Option<Position> {
        if self.1 == 3 {
            return None;
        }
        let hash = lo_nib(md5::compute(&self.2)[0]);
        if is_open(hash) {
            Some(Position(self.0, self.1 + 1, format!("{}D", self.2)))
        } else {
            None
        }
    }

    fn move_up(&self) -> Option<Position> {
        if self.1 == 0 {
            return None;
        }
        let hash = hi_nib(md5::compute(&self.2)[0]);
        if is_open(hash) {
            Some(Position(self.0, self.1 - 1, format!("{}U", self.2)))
        } else {
            None
        }
    }

    fn move_left(&self) -> Option<Position> {
        if self.0 == 0 {
            return None;
        }
        let hash = hi_nib(md5::compute(&self.2)[1]);
        if is_open(hash) {
            Some(Position(self.0 - 1, self.1, format!("{}L", self.2)))
        } else {
            None
        }
    }

    fn move_right(&self) -> Option<Position> {
        if self.0 == 3 {
            return None;
        }
        let hash = lo_nib(md5::compute(&self.2)[1]);
        if is_open(hash) {
            Some(Position(self.0 + 1, self.1, format!("{}R", self.2)))
        } else {
            None
        }
    }

    fn neighbors(&self) -> SmallVec<[Position; 4]> {
        let mut v = SmallVec::new();
        v.extend(self.move_down());
        v.extend(self.move_up());
        v.extend(self.move_left());
        v.extend(self.move_right());
        v
    }
}

pub fn part1(s: &str) -> String {
    let pos = Position(0, 0, s.to_string());

    let mut queue = BinaryHeap::new();
    let mut min_dist = usize::max_value();
    queue.push(Reverse(pos));
    let mut min_path = None::<String>;
    while let Some(Reverse(nxt)) = queue.pop() {
        if nxt.2.len() > min_dist {
            continue;
        }
        if nxt.0 == 3 && nxt.1 == 3 {
            min_dist = min(nxt.2.len(), min_dist);
            if min_path.is_none() || nxt.2.len() < min_path.as_ref().unwrap().len() {
                min_path = Some(nxt.2);
            }
            continue;
        }
        for neighbor in nxt.neighbors() {
            queue.push(Reverse(neighbor));
        }
    }
    min_path.unwrap()[s.len()..].to_string()
}

pub fn part2(s: &str) -> usize {
    let pos = Position(0, 0, s.to_string());

    let mut queue = BinaryHeap::new();
    let mut max_dist = 0;
    queue.push(pos);
    while let Some(nxt) = queue.pop() {
        if nxt.0 == 3 && nxt.1 == 3 {
            max_dist = max(nxt.2.len(), max_dist);
            continue;
        }
        for neighbor in nxt.neighbors() {
            queue.push(neighbor);
        }
    }

    max_dist - s.len()
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), "RLRDRDUDDR");
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 420);
}

pub static INPUT: &str = "rrrbmfta";
