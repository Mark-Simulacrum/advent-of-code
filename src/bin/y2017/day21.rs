#![allow(unused)]

use std::marker::PhantomData;
use std::mem;
use std::fmt;
use std::iter;
use smallvec::SmallVec;

const ON: u8 = b'#';
const OFF: u8 = b'.';

#[derive(Debug, Clone, PartialEq, Eq)]
struct BitVec(SmallVec<[u64; 1]>);

impl BitVec {
    const BITS_PER_CELL: usize = mem::size_of::<u64>() * 8;
    fn new() -> Self {
        BitVec(SmallVec::from([0; 1]))
    }

    fn count_ones(&self) -> u32 {
        self.0.iter().map(|c| c.count_ones()).sum()
    }

    fn get_cell_mut(&mut self, i: usize) -> &mut u64 {
        let index = i / Self::BITS_PER_CELL;
        while index >= self.0.len() { self.0.push(0); }
        &mut self.0[index]
    }

    fn get_cell(&self, i: usize) -> u64 {
        let index = i / Self::BITS_PER_CELL;
        self.0.get(index).cloned().unwrap_or(0)
    }

    fn bit_in_cell(i: usize) -> usize {
        i % Self::BITS_PER_CELL
    }

    fn set(&mut self, i: usize, v: bool) {
        let cell = self.get_cell_mut(i);
        let mask = 1u64 << Self::bit_in_cell(i);
        *cell &= !mask;
        if v {
            *cell |= mask;
        }
    }

    fn get(&self, i: usize) -> bool {
        (self.get_cell(i) & (1 << Self::bit_in_cell(i))) != 0
    }

    fn make_space_at(&mut self, i: usize) {
        let bit = Self::bit_in_cell(i);
        let last_idx = (i / Self::BITS_PER_CELL + 1) * Self::BITS_PER_CELL - 1;
        let last_bit_in_cell = self.get(last_idx);
        if last_bit_in_cell {
            self.insert(last_idx + 1, true);
        }
        let right_mask = 2u64.pow(bit as u32) - 1;
        let right = self.get_cell(i) & right_mask;
        let left = self.get_cell(i) & !right_mask;
        {
            let cell = self.get_cell_mut(i);
            *cell = ((left << 1) | right);
        }
    }

    fn insert(&mut self, i: usize, v: bool) {
        self.make_space_at(i);
        self.set(i, v);
    }
}

#[test]
fn bitvec_insert() {
    let mut v = BitVec::new();
    v.set(0, true);
    v.set(2, true);
    v.insert(1, true);
    assert_eq!(format!("{:05b}", v.0[0]), format!("{:05b}", 0b1011));
}

#[test]
fn bitvec_insert_at_end() {
    let mut v = BitVec::new();
    v.set(0, true);
    v.set(2, true);
    v.insert(63, true);
    v.insert(63, true);
    assert_eq!(format!("{:064b}", v.0[0]), format!("{:064b}", (0b1u64 << 63) | 0b101));
    assert_eq!(format!("{:064b}", v.0[1]), format!("{:064b}", 0b1));
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    grid: BitVec,
    size: usize,
}

struct DebugGrid<'a, 'b>(&'a Grid, &'b str);

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", DebugGrid(self, "/"))
    }
}

impl<'a, 'b> fmt::Debug for DebugGrid<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.0.size {
            for j in 0..self.0.size {
                let idx = i*self.0.size + j;
                if self.0.grid.get(idx) {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }
            }
            if i + 1 != self.0.size {
                write!(f, "{}", self.1)?;
            }
        }
        Ok(())
    }
}

#[test]
fn grid_dbg_1() {
    assert_eq!(format!("{:?}", Grid::interpret("../..")), "../..");
}

#[test]
fn grid_dbg_2() {
    assert_eq!(format!("{:?}", Grid::interpret(".#/#.")), ".#/#.");
}

#[test]
fn grid_dbg_3() {
    assert_eq!(format!("{:?}", Grid::interpret(".#./..#/...")), ".#./..#/...");
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            grid: BitVec::new(),
            size,
        }
    }

    fn interpret(pattern: &str) -> Grid {
        let mut grid = BitVec::new();
        let mut size = 0;
        let mut out = 0;
        let mut idx = 0;
        for (i, &b) in pattern.trim().as_bytes().iter().enumerate() {
            if b == b'/' || b == b'\n' {
                if size == 0 {
                    size = i;
                }
                continue;
            }
            assert!(b == ON || b == OFF, "unexpected byte {:?} in {:?}", b as char, pattern);
            grid.set(idx, b == ON);
            idx += 1;
        }
        assert_ne!(size, 0);
        Grid {
            grid,
            size,
        }
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.grid.get(row*self.size + col)
    }

    fn set(&mut self, row: usize, col: usize, v: bool) {
        self.grid.set(row*self.size + col, v)
    }

    fn transpose(&mut self) {
        let mut grid = Grid::new(self.size);
        for row in 0..self.size {
            let x = (0..self.size)
                .into_iter()
                .map(|c| self.get(row, c))
                .collect::<SmallVec<[_; 5]>>();
            for (col, v) in x.into_iter().enumerate() {
                grid.set(col, row, v);
            }
        }
        *self = grid;
    }

    fn reverse_rows(&mut self) {
        let s = self.size;
        for row in 0..self.size {
            let x = (0..self.size)
                .into_iter()
                .rev()
                .map(|c| self.get(row, c))
                .collect::<SmallVec<[_; 5]>>();
            for (i, v) in x.into_iter().enumerate() {
                self.set(row, i, v);
            }
        }
    }

    fn rotate_90(&mut self) {
        self.transpose();
        self.reverse_rows();
    }

    fn reversed_rows(&self) -> Grid {
        let mut a = self.clone();
        a.reverse_rows();
        a
    }

    fn bump_size(&mut self, d: usize) {
        assert!(self.size % d == 0);
        let mut increased_by = 0;
        for i in 0..(self.size / d) {
            self.grid.insert(increased_by + i*d+1, false);
            increased_by += 1;
        }
        self.size += self.size / d;
    }

    fn matches(&mut self, other: &Self) -> bool {
        assert!(self.size == 2 || self.size == 3,
            "can only match 2 or 3 sized patterns, found: {}", self.size);
        assert_eq!(self.size, other.size);
        if self == other { return true; }
        let mut a = self;
        if a.reversed_rows() == *other { return true; }
        a.rotate_90();
        if a == other { return true; }
        if a.reversed_rows() == *other { return true; }
        a.rotate_90(); // 180
        if a == other { return true; }
        if a.reversed_rows() == *other { return true; }
        a.rotate_90(); // 270
        if a == other { return true; }
        if a.reversed_rows() == *other { return true; }
        false
    }

    fn load_from(&mut self, start: usize, other: &Grid) {
        for row in 0..self.size {
            for col in 0..self.size {
                let o = other.grid.get(start + row*other.size + col);
                self.set(row, col, o);
            }
        }
    }

    fn set_from(&mut self, start: usize, other: &Grid) {
        for row in 0..other.size {
            for col in 0..other.size {
                let o = other.grid.get(row*other.size + col);
                let idx = start + row*self.size + col;
                self.grid.set(idx, o);
            }
        }
    }

    fn step(&mut self, n: usize, rules: &[(Grid, Grid)]) {
        let mut outs = Vec::new();
        for col in 0..(self.size / n) {
            for row in 0..(self.size / n) {
                let idx = col * n * self.size + row * n;
                let mut grid = Grid::new(n);
                grid.load_from(idx, &self);
                for &(ref rule, ref out) in rules {
                    if grid.matches(rule) {
                        outs.push(out);
                    }
                }
            }
        }
        self.bump_size(n);
        let size = self.size;
        let m = n+1;
        for i in 0..(size / m) {
            for j in 0..(size / m) {
                self.set_from(i*m*size + j*m, outs.remove(0));
            }
        }
        assert_eq!(outs.len(), 0);
    }
}

#[test]
fn grid_interpret() {
    assert_eq!(Grid::interpret(".#/.#"), Grid {
        grid: BitVec(SmallVec::from(vec![0b1010])),
        size: 2,
    });
}

#[test]
fn grid_reverse_rows() {
    let mut input = Grid::interpret(".#/.#");
    input.reverse_rows();
    assert_eq!(input, Grid::interpret("#./#."));
}

#[test]
fn grid_transpose() {
    let mut input = Grid::interpret(".#/.#");
    input.transpose();
    assert_eq!(input, Grid::interpret("../##"));
}

#[test]
fn grid_rotate90() {
    let mut input = Grid::interpret(".#/.#");
    input.rotate_90();
    assert_eq!(input, Grid::interpret("../##"));
}

#[test]
fn grid_bump_size_1() {
    let mut grid = Grid::interpret("../.#");
    grid.bump_size(2);
    assert_eq!(grid, Grid::interpret(".../.#./..."));
}

fn eval(s: &str, iterations: usize) -> u32 {
    let (rules_2, rules_3): (Vec<(Grid, Grid)>, _) = s.trim().lines().flat_map(|line| {
        let mut parts = line.split(" => ");
        let pattern = Grid::interpret(parts.next().unwrap());
        let output = Grid::interpret(parts.next().unwrap());
        iter::once((pattern, output))
    }).partition(|r| r.0.size == 2);
    let mut state = Grid::interpret(INITIAL_PATTERN.trim());
    for iteration in 0..iterations {
        if state.size % 2 == 0 {
            state.step(2, &rules_2);
        } else {
            assert!(state.size % 3 == 0);
            state.step(3, &rules_3);
        }
        println!("i = {}, size={}, elements={}", iteration, state.size, state.grid.0.len());
    }
    state.grid.count_ones()
}

pub fn part1(s: &str) -> u32 {
    eval(s, 5)
}

pub fn part2(s: &str) -> u32 {
    eval(s, 18)
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 176);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 2368161);
}

static INITIAL_PATTERN: &str = "
.#.
..#
###
";

pub static INPUT: &str = "
../.. => .#./.../###
#./.. => .#./##./#..
##/.. => #.#/#../###
.#/#. => ##./..#/###
##/#. => .#./#../..#
##/## => #../..#/#.#
.../.../... => .###/.#.#/.###/##.#
#../.../... => .##./##../##../#.##
.#./.../... => .#.#/#.#./..#./..#.
##./.../... => ###./#.##/...#/#.##
#.#/.../... => .#.#/.#../.###/.###
###/.../... => ..##/#.#./..../##.#
.#./#../... => #.../..../..../....
##./#../... => ...#/..#./.###/#.#.
..#/#../... => #.../#.##/###./##..
#.#/#../... => .##./#..#/#..#/..##
.##/#../... => .#.#/#.##/..##/.#.#
###/#../... => #.#./.###/..#./#.#.
.../.#./... => #..#/..../.##./.#.#
#../.#./... => .#../.##./.#.#/...#
.#./.#./... => ##.#/...#/.##./...#
##./.#./... => ..#./#.#./#.##/####
#.#/.#./... => ..##/#..#/.###/....
###/.#./... => .#../#..#/#.../..#.
.#./##./... => ..##/#.#./####/###.
##./##./... => ...#/.#../####/#..#
..#/##./... => ..##/##../###./....
#.#/##./... => ..##/#.../.#../.##.
.##/##./... => #.../##../#.##/...#
###/##./... => .#../####/#.##/#.##
.../#.#/... => #..#/####/###./#.#.
#../#.#/... => #.../##.#/#.../.#..
.#./#.#/... => ##.#/##.#/..#./..#.
##./#.#/... => .###/..#./.#../.###
#.#/#.#/... => .###/##../..#./..#.
###/#.#/... => ##../.#../.#../.#..
.../###/... => ..#./#.#./..#./#..#
#../###/... => ..../#.#./##.#/..##
.#./###/... => ..#./#.#./..##/.#..
##./###/... => .##./..##/#..#/#.#.
#.#/###/... => ###./###./#.##/..##
###/###/... => ##.#/..../.##./.#..
..#/.../#.. => .###/####/..../##.#
#.#/.../#.. => ##../###./#..#/...#
.##/.../#.. => ###./#..#/###./...#
###/.../#.. => #.../#..#/##.#/.##.
.##/#../#.. => ..##/####/..##/#...
###/#../#.. => #.../..../...#/..##
..#/.#./#.. => ####/#.#./..../.#.#
#.#/.#./#.. => .##./.#.#/##.#/.##.
.##/.#./#.. => ###./.#.#/###./##.#
###/.#./#.. => #.##/..##/#.#./##.#
.##/##./#.. => ..../..##/#.#./.##.
###/##./#.. => #.#./#..#/#..#/###.
#../..#/#.. => ..../####/#..#/.###
.#./..#/#.. => .###/#.../#.../#.##
##./..#/#.. => ####/##.#/###./####
#.#/..#/#.. => .#../##.#/#..#/#..#
.##/..#/#.. => ..##/##.#/#.##/###.
###/..#/#.. => ##.#/####/##.#/.#..
#../#.#/#.. => .###/#..#/.##./.###
.#./#.#/#.. => #.##/.##./.#../..#.
##./#.#/#.. => ###./..#./.##./##..
..#/#.#/#.. => .###/.#.#/#.#./##..
#.#/#.#/#.. => #..#/.###/.##./....
.##/#.#/#.. => ###./.###/#.##/.###
###/#.#/#.. => ####/.###/..../.##.
#../.##/#.. => ##.#/..../#.../..#.
.#./.##/#.. => #.../..../...#/###.
##./.##/#.. => ###./.#../..##/...#
#.#/.##/#.. => #.../...#/..#./.###
.##/.##/#.. => ###./..../##.#/...#
###/.##/#.. => ##.#/##../###./.##.
#../###/#.. => ..#./#.../..##/#.##
.#./###/#.. => ...#/.##./.#../.#..
##./###/#.. => ##.#/.#.#/###./....
..#/###/#.. => #.##/#.../####/.##.
#.#/###/#.. => .#.#/...#/#..#/..#.
.##/###/#.. => .##./#..#/#..#/.#.#
###/###/#.. => ###./####/#.##/#...
.#./#.#/.#. => ###./#..#/...#/...#
##./#.#/.#. => #.#./#.##/#.../#..#
#.#/#.#/.#. => .#.#/#.##/..../.#..
###/#.#/.#. => #.#./.#../.###/#.#.
.#./###/.#. => #.../.###/##../##.#
##./###/.#. => .###/#.../####/.#.#
#.#/###/.#. => #..#/####/#.#./#...
###/###/.#. => .#../..../.##./.#.#
#.#/..#/##. => ##../###./...#/###.
###/..#/##. => .##./###./.###/#.##
.##/#.#/##. => ..../##.#/#..#/#...
###/#.#/##. => .###/##../..../..#.
#.#/.##/##. => ####/.###/##../...#
###/.##/##. => #.##/..##/..#./#..#
.##/###/##. => ..../#.##/#.../#.##
###/###/##. => ..../#..#/#.##/#.##
#.#/.../#.# => #.../##.#/..../.#.#
###/.../#.# => ##../##../#.#./.##.
###/#../#.# => .##./.#../#.##/.##.
#.#/.#./#.# => #.../.#../####/#.##
###/.#./#.# => .###/##.#/#.../#.#.
###/##./#.# => .##./.##./.###/.#.#
#.#/#.#/#.# => ####/####/###./.##.
###/#.#/#.# => #.#./.###/...#/.#.#
#.#/###/#.# => .###/..#./..../.##.
###/###/#.# => #.#./##.#/..#./..#.
###/#.#/### => ###./#.../##../##..
###/###/### => ##.#/.#.#/#.#./...#
";
