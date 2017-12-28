use std::mem;
use std::fmt;
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

    fn with_capacity(n: usize) -> Self {
        BitVec(SmallVec::from_vec(vec![0; n / Self::BITS_PER_CELL]))
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
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    grid: BitVec,
    size: usize,
    iterations: usize,
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
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
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
            grid: BitVec::with_capacity(size.pow(2)),
            size,
            iterations: 0,
        }
    }

    fn interpret(pattern: &str) -> Grid {
        let mut grid = BitVec::new();
        let mut size = 0;
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
            iterations: 0,
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

    fn step(&self, rules_2: Rules, rules_3: Rules) -> Grid {
        if self.size % 2 == 0 {
            self.step_n(2, rules_2)
        } else {
            assert!(self.size % 3 == 0);
            self.step_n(3, rules_3)
        }
    }

    fn step_n(&self, n: usize, rules: &[(Vec<Grid>, Grid)]) -> Grid {
        let mut next = Grid::new(self.size + (self.size / n));
        next.iterations = self.iterations + 1;
        for row in 0..(self.size / n) {
            for col in 0..(self.size / n) {
                let idx = row * n * self.size + col * n;
                let mut grid = Grid::new(n);
                grid.load_from(idx, &self);
                for &(ref rules, ref out) in rules {
                    for rule in rules {
                        if grid == *rule {
                            let m = n + 1;
                            let s = next.size;
                            next.set_from(
                                row * m * s +
                                col * m, out);
                            break;
                        }
                    }
                }
            }
        }
        next
    }
}

type Rules<'a> = &'a [(Vec<Grid>, Grid)];

#[test]
fn grid_interpret() {
    assert_eq!(Grid::interpret(".#/.#"), Grid {
        grid: BitVec(SmallVec::from(vec![0b1010])),
        size: 2,
        iterations: 0,
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

fn eval(s: &str, iterations: usize) -> u32 {
    let (rules_2, rules_3): (Vec<(Vec<Grid>, Grid)>, _) = s.trim().lines().map(|line| {
        let mut parts = line.split(" => ");
        let mut pattern = Grid::interpret(parts.next().unwrap());
        let output = Grid::interpret(parts.next().unwrap());
        let mut patterns = Vec::with_capacity(1);
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        pattern.rotate_90(); // 90
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        pattern.rotate_90(); // 180
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        pattern.rotate_90(); // 270
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        (patterns, output)
    }).partition(|r| r.0[0].size == 2);
    let mut grids = vec![Grid::interpret(INITIAL_PATTERN.trim())];
    let mut total = 0;
    while let Some(grid) = grids.pop() {
        if grid.iterations == iterations {
            total += grid.grid.count_ones();
            continue;
        }
        let state = grid.step(&rules_2, &rules_3);
        if state.size == 3usize.pow(3) {
            let m = 3usize.pow(2); // previous power of 3 from state.size
            let mut next = vec![Grid::new(m); 9];
            for row in 0..3 {
                for col in 0..3 {
                    next[row * 3 + col].load_from(row*m*state.size + col*m, &state);
                    next[row * 3 + col].iterations = state.iterations;
                }
            }
            grids.extend(next);
        } else {
            grids.push(state);
        }
    }
    total
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
