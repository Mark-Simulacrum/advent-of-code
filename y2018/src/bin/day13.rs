use aoc_macro::{generator, solution};
use fnv::FnvHashMap as HashMap;
use fnv::FnvHashSet as HashSet;
use std::collections::BTreeMap;
use std::ops;

aoc_macro::day!();

type Out = State;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}

struct State {
    corners: HashMap<CartPosition, Corner>,
    intersections: HashSet<CartPosition>,
    carts: CartMap,
}

#[derive(Default)]
struct CartMap(BTreeMap<CartPosition, Cart>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CartPosition {
    // note that y must be first because they must be ordered by row and then column
    y: usize,
    x: usize,
}

impl CartMap {
    fn insert(&mut self, position: CartPosition, facing: Direction) {
        self.0.insert(
            position,
            Cart {
                position,
                facing,
                turn: 0,
            },
        );
    }
}

/// Mapping from incoming direction to outgoing direction
#[derive(Copy, Clone, Debug)]
struct Corner {
    up: Direction,
    down: Direction,
    left: Direction,
    right: Direction,
}

impl ops::Index<Direction> for Corner {
    type Output = Direction;
    fn index(&self, index: Direction) -> &Direction {
        match index {
            Direction::Left => &self.left,
            Direction::Down => &self.down,
            Direction::Right => &self.right,
            Direction::Up => &self.up,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Cart {
    position: CartPosition,
    facing: Direction,
    turn: u8,
}

#[generator]
fn generator(input: &str) -> Out {
    let input = if input.starts_with("\n") {
        &input[1..]
    } else {
        input
    };
    let mut carts = CartMap::default();
    let mut corners = fnv::FnvHashMap::default();
    let mut intersections = HashSet::default();
    for (line_idx, line) in input.lines().enumerate() {
        for (ch_idx, ch) in line.as_bytes().iter().enumerate() {
            let ch = *ch as char;
            if ch == ' ' {
                continue;
            }

            let cur = CartPosition {
                x: ch_idx,
                y: line_idx,
            };
            match ch {
                '-' | '>' | '<' => {}
                '|' | '^' | 'v' => {}
                '\\' => {
                    corners.insert(
                        cur,
                        Corner {
                            up: Direction::Left,
                            down: Direction::Right,
                            left: Direction::Up,
                            right: Direction::Down,
                        },
                    );
                }
                '/' => {
                    corners.insert(
                        cur,
                        Corner {
                            up: Direction::Right,
                            down: Direction::Left,
                            left: Direction::Down,
                            right: Direction::Up,
                        },
                    );
                }
                '+' => {
                    intersections.insert(cur);
                }
                _ => panic!("unexpected character: {}", ch),
            }

            let dir = match ch {
                '>' => Direction::Right,
                '<' => Direction::Left,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => continue,
            };
            carts.insert(cur, dir);
        }
    }

    State {
        carts,
        corners,
        intersections,
    }
}

impl Cart {
    fn next(&mut self, state: &State) {
        if let Some(corner) = state.corners.get(&self.position) {
            self.facing = corner[self.facing];
        }

        if state.intersections.contains(&self.position) {
            if self.turn == 3 {
                self.turn = 0;
            }
            self.facing = match self.turn {
                0 => self.facing.turn_left(),
                1 => self.facing,
                2 => self.facing.turn_right(),
                _ => panic!("too many intersections"),
            };
            self.turn += 1;
        }

        let CartPosition { x, y } = self.position;
        let (nx, ny) = match self.facing {
            Direction::Down => (x, y + 1),
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        self.position = CartPosition { x: nx, y: ny };
    }
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = (7, 3),
    expect = (91, 69))]
fn part1(mut input: State) -> (usize, usize) {
    loop {
        let map = std::mem::replace(&mut input.carts.0, BTreeMap::new());
        let mut previous = map.keys().cloned().collect::<HashSet<_>>();
        let mut next = CartMap::default();
        for (_, mut cart) in map {
            let start = cart.position;
            cart.next(&input);
            let pos = cart.position;
            if previous.contains(&pos) || next.0.contains_key(&pos) {
                // collision!
                return (pos.x, pos.y);
            }
            previous.remove(&start);
            next.0.insert(pos, cart);
        }
        input.carts = next;
    }
}

#[solution(part2,
    example_input = generator(EXAMPLE_2),
    example = (6, 4),
    expect = (44, 87))]
fn part2(mut input: Out) -> (usize, usize) {
    loop {
        let map = std::mem::replace(&mut input.carts.0, BTreeMap::new());
        let mut previous = map.keys().cloned().collect::<HashSet<_>>();
        let mut next = CartMap::default();
        for (_, mut cart) in map {
            let start = cart.position;
            if !previous.contains(&start) {
                // previously deleted this cart (due to collision); skip
                continue;
            }
            cart.next(&input);
            let pos = cart.position;
            if previous.contains(&pos) || next.0.contains_key(&pos) {
                // collision; remove from both lists
                previous.remove(&pos);
                next.0.remove(&pos);
                continue;
            }
            previous.remove(&start);
            next.0.insert(pos, cart);
        }
        input.carts = next;
        if input.carts.0.len() == 1 {
            let pos = input.carts.0.values().next().unwrap().position;
            return (pos.x, pos.y);
        }
        eprintln!("{:?}", input.carts.0.len());
    }
}

static EXAMPLE: &str = r"
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
";
static EXAMPLE_2: &str = r"
/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
";
static INPUT: &str = include_str!("day13.input");
