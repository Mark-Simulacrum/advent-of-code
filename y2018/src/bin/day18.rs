use aoc_macro::{generator, solution};
use hashbrown::HashMap;
use std::fmt;

aoc_macro::day!();

type Out = State;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn add_x(self, dx: isize) -> Option<Position> {
        if dx < 0 {
            Some(Position { x: self.x.checked_sub(dx.abs() as usize)?, y: self.y })
        } else {
            Some(Position { x: self.x.checked_add(dx as usize)?, y: self.y })
        }
    }

    fn add_y(self, dy: isize) -> Option<Position> {
        if dy < 0 {
            Some(Position { y: self.y.checked_sub(dy.abs() as usize)?, x: self.x })
        } else {
            Some(Position { y: self.y.checked_add(dy as usize)?, x: self.x })
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let pos = Position { x, y };
                let c = match self.map[&pos] {
                    Acre::Lumberyard => '#',
                    Acre::Open => '.',
                    Acre::Trees => '|',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[generator]
fn generator(input: &str) -> Out {
    let mut map = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (line_idx, line) in input.trim().lines().enumerate() {
        for (ch_idx, ch) in line.chars().enumerate() {
            let pos = Position { y: line_idx, x: ch_idx };
            max_x = std::cmp::max(max_x, pos.x);
            max_y = std::cmp::max(max_y, pos.y);
            map.insert(pos, match ch {
                '#' => Acre::Lumberyard,
                '|' => Acre::Trees,
                '.' => Acre::Open,
                _ => panic!("unknown: {:?}", ch),
            });
        }
    }
    State {
        max_x,
        max_y,
        map,
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Counts {
    open: usize,
    trees: usize,
    lumber: usize,
}

impl State {
    fn counts(&self, pos: Position) -> Counts {
        let mut counts = Counts::default();
        for dx in -1..=1 {
            if let Some(pos) = pos.add_x(dx) {
                for dy in -1..=1 {
                    if let Some(pos) = pos.add_y(dy) {
                        if dx == 0 && dy == 0 { continue; }
                        match self.map.get(&pos).cloned() {
                            Some(Acre::Open) => counts.open += 1,
                            Some(Acre::Trees) => counts.trees += 1,
                            Some(Acre::Lumberyard) => counts.lumber += 1,
                            None => {},
                        }
                    }
                }
            }
        }
        counts
    }

    fn tick(&mut self) {
        let prev = self.clone();
        for (pos, acre) in &mut self.map {
            let c = prev.counts(*pos);
            match acre {
                Acre::Open => {
                    if c.trees >= 3 {
                        *acre = Acre::Trees;
                    }
                }
                Acre::Trees => {
                    if c.lumber >= 3 {
                        *acre = Acre::Lumberyard;
                    }
                }
                Acre::Lumberyard => {
                    if c.lumber >= 1 && c.trees >= 1 {
                        // remains
                    } else {
                        *acre = Acre::Open;
                    }
                }
            }
        }
    }

    fn resource_value(&self) -> usize {
        self.map.values().filter(|a| **a == Acre::Trees).count() *
        self.map.values().filter(|a| **a == Acre::Lumberyard).count()
    }
}

#[derive(Clone)]
struct State {
    map: HashMap<Position, Acre>,
    max_x: usize,
    max_y: usize,
}

impl State {
    fn save(&self) -> SavedState {
        let mut v: Vec<_> = self.map.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort_unstable();
        SavedState {
            map: v,
        }
    }
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 1147,
    expect = 574200)]
fn part1(mut state: Out) -> usize {
    for _ in 0..10 {
        state.tick();
    }
    state.resource_value()
}

#[derive(PartialEq, Hash, Eq)]
struct SavedState {
    map: Vec<(Position, Acre)>,
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0)]
fn part2(mut state: Out) -> usize {
    let end = 1_000_000_000;
    let mut seen: HashMap<SavedState, usize> = HashMap::new();
    let mut i = 0;
    while i < end {
        state.tick();
        i += 1;
        let save = state.save();
        if let Some(&idx) = seen.get(&save) {
            let period = i - idx;
            let periods_left = (end - i) / period;
            i += period * periods_left;
            continue;
        } else {
            seen.insert(save, i);
        }
    }
    state.resource_value()
}

static EXAMPLE: &str = "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
";
static INPUT: &str = include_str!("day18.input");
