use advent_of_code::{BitVec, Grid, Matrix, VecLike};

#[derive(Copy, Clone, Debug)]
enum NodeState {
    Clean,
    Weak,
    Infected,
    Flagged,
}

impl Default for NodeState {
    fn default() -> NodeState {
        NodeState::Clean
    }
}

fn load<V: Copy + Default, C: VecLike<V>>(s: &str, empty: V, present: V) -> Grid<V, C> {
    let matrix = Matrix::interpret(s, empty, present);
    let lx = -(matrix.columns() as isize / 2);
    let ly = -(matrix.rows() as isize / 2);
    Grid::from_matrix(matrix, (lx, ly))
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    #[inline(always)]
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[inline(always)]
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    #[inline(always)]
    fn apply(self, pos: &mut (isize, isize)) {
        match self {
            Direction::Up => pos.1 -= 1,
            Direction::Down => pos.1 += 1,
            Direction::Left => pos.0 -= 1,
            Direction::Right => pos.0 += 1,
        }
    }
}

pub fn part1(s: &str) -> usize {
    let mut grid = load::<bool, BitVec>(s, false, true);
    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    let mut infections = 0;
    for _ in 0..10_000 {
        let value = grid.get(pos.0, pos.1);
        grid.set(pos.0, pos.1, !value);
        if value {
            dir = dir.turn_right();
        } else {
            infections += 1;
            dir = dir.turn_left();
        }
        dir.apply(&mut pos);
    }
    infections
}

pub fn part2(s: &str) -> usize {
    let mut grid = load::<NodeState, Vec<_>>(s, NodeState::Clean, NodeState::Infected);
    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    let mut infections = 0;
    for _ in 0..10_000_000 {
        let next = match grid.get(pos.0, pos.1) {
            NodeState::Clean => {
                dir = dir.turn_left();
                NodeState::Weak
            }
            NodeState::Weak => {
                infections += 1;
                NodeState::Infected
            }
            NodeState::Infected => {
                dir = dir.turn_right();
                NodeState::Flagged
            }
            NodeState::Flagged => {
                dir = dir.reverse();
                NodeState::Clean
            }
        };
        grid.set(pos.0, pos.1, next);
        dir.apply(&mut pos);
    }
    infections
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 5587);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 5575);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE), 2511944);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 2511991);
}

#[cfg(test)]
static EXAMPLE: &str = "
.........
.........
.........
.....#...
...#.....
.........
.........
.........
";

pub static INPUT: &str = "
.......##.#..####.#....##
..###....###.####..##.##.
#..####.#....#.#....##...
.#....#.#.#....#######...
.###..###.#########....##
##...#####..#####.###.#..
.#..##.###.#.#....######.
.#.##.#..####..#.##.....#
#.#..###..##..#......##.#
##.###.##.#.#...##.#.##..
##...#.######.#..##.#...#
....#.####..#..###.##..##
...#....#.###.#.#..#.....
..###.#.#....#.....#.####
.#....##..##.#.#...#.#.#.
...##.#.####.###.##...#.#
##.#.####.#######.##..##.
.##...#......####..####.#
#..###.#.###.##.#.#.##..#
#..###.#.#.#.#.#....#.#.#
####.#..##..#.#..#..#.###
##.....#..#.#.#..#.####..
#####.....###.........#..
##...#...####..#####...##
.....##.#....##...#.....#
";
