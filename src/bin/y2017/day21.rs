use std::fmt;

use advent_of_code::{Matrix, BitVec};

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    matrix: Matrix<bool, BitVec>,
    size: usize,
    iterations: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.matrix)
    }
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            matrix: Matrix::new(size, size),
            size: size,
            iterations: 0,
        }
    }

    fn interpret(pattern: &str) -> Grid {
        let matrix = Matrix::interpret(pattern, false, true);
        assert_eq!(matrix.columns(), matrix.rows());
        Grid {
            size: matrix.columns(),
            matrix,
            iterations: 0,
        }
    }

    fn reversed_rows(&self) -> Grid {
        let mut a = self.clone();
        a.matrix.reverse_rows();
        a
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
                grid.matrix.load_from(idx, &self.matrix);
                for &(ref rules, ref out) in rules {
                    for rule in rules {
                        if grid == *rule {
                            let m = n + 1;
                            let s = next.size;
                            next.matrix.set_from(
                                row * m * s +
                                col * m, &out.matrix);
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

fn eval(s: &str, iterations: usize) -> u32 {
    let (rules_2, rules_3): (Vec<(Vec<Grid>, Grid)>, _) = s.trim().lines().map(|line| {
        let mut parts = line.split(" => ");
        let mut pattern = Grid::interpret(parts.next().unwrap());
        let output = Grid::interpret(parts.next().unwrap());
        let mut patterns = Vec::with_capacity(1);
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        pattern.matrix.rotate_90(); // 90
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        pattern.matrix.rotate_90(); // 180
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        pattern.matrix.rotate_90(); // 270
        patterns.push(pattern.clone());
        patterns.push(pattern.reversed_rows());
        (patterns, output)
    }).partition(|r| r.0[0].size == 2);
    let mut grids = vec![Grid::interpret(INITIAL_PATTERN.trim())];
    let mut total = 0;
    while let Some(grid) = grids.pop() {
        if grid.iterations == iterations {
            total += grid.matrix.count_set();
            continue;
        }
        let state = grid.step(&rules_2, &rules_3);
        if state.size == 3usize.pow(3) {
            let m = 3usize.pow(2); // previous power of 3 from state.size
            let mut next = vec![Grid::new(m); 9];
            for row in 0..3 {
                for col in 0..3 {
                    next[row * 3 + col].matrix.load_from(row*m*state.size + col*m, &state.matrix);
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
