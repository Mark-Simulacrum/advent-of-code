use aoc_macro::{generator, sol_test, solution};
use hashbrown::{HashMap, HashSet};
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;
use petgraph::Graph;
use std::cmp::{self, Ordering};
use std::collections::BTreeMap;
use std::fmt::{self, Write};

aoc_macro::day!();

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    y: usize,
    x: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn up(self) -> Position {
        let y = self.y.checked_sub(1).unwrap();
        Position { y, x: self.x }
    }
    fn down(self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(self) -> Position {
        Position {
            x: self.x.checked_sub(1).unwrap(),
            y: self.y,
        }
    }
    fn right(self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn cross(self) -> [Position; 4] {
        [self.up(), self.left(), self.right(), self.down()]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Goblin(usize, bool),
    Elf(usize, bool),
    Wall,
    Empty,
}

impl Cell {
    // returns enemy
    fn attack(self, enemy: Cell, elf_power: usize) -> Option<Cell> {
        let r = match enemy {
            Cell::Goblin(h, v) => Cell::Goblin(h.saturating_sub(elf_power), v),
            Cell::Elf(h, v) => Cell::Elf(h.saturating_sub(3), v),
            Cell::Wall | Cell::Empty => panic!("cannot attack {:?}", self),
        };
        if r.health() == 0 {
            None
        } else {
            Some(r)
        }
    }

    fn is_enemy(self, enemy: Cell) -> bool {
        match (self, enemy) {
            (Cell::Elf(..), Cell::Goblin(..)) => true,
            (Cell::Goblin(..), Cell::Elf(..)) => true,
            _ => false,
        }
    }

    fn visited(self) -> bool {
        match self {
            Cell::Goblin(_, visited) | Cell::Elf(_, visited) => visited,
            Cell::Wall | Cell::Empty => false,
        }
    }

    fn set_visited(&mut self, v: bool) {
        match self {
            Cell::Goblin(_, visited) | Cell::Elf(_, visited) => {
                *visited = v;
            }
            Cell::Wall | Cell::Empty => {}
        }
    }

    fn health(self) -> usize {
        match self {
            Cell::Goblin(h, ..) => h,
            Cell::Elf(h, ..) => h,
            Cell::Wall | Cell::Empty => panic!("cannot get health of {:?}", self),
        }
    }

    fn is_goblin(self) -> bool {
        if let Cell::Goblin(..) = self {
            true
        } else {
            false
        }
    }
    fn is_elf(self) -> bool {
        if let Cell::Elf(..) = self {
            true
        } else {
            false
        }
    }
    fn is_empty(self) -> bool {
        self == Cell::Empty
    }
}

#[derive(Clone)]
struct State {
    cells: BTreeMap<Position, Cell>,
    elf_power: usize,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut last_y = 0;
        let mut to_print = String::new();
        for (pos, cell) in &self.cells {
            if pos.y != last_y {
                last_y = pos.y;
                write!(f, " {}\n", to_print)?;
                to_print.clear();
            }
            let c = match cell {
                Cell::Goblin(h, _) => {
                    write!(to_print, "G({}) ", h)?;
                    "G"
                }
                Cell::Elf(h, _) => {
                    write!(to_print, "E({}) ", h)?;
                    "E"
                }
                Cell::Empty => ".",
                Cell::Wall => "#",
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Candidate {
    Path {
        from: Position,
        to: Position,
        enemy: Position,
        distance: usize,
        next: Position,
    },
    Immediate {
        us: Position,
        enemy: Position,
    },
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Candidate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Candidate) -> Ordering {
        match (self, other) {
            (
                Candidate::Immediate { us: u1, enemy: e1 },
                Candidate::Immediate { us: u2, enemy: e2 },
            ) => {
                assert_eq!(u1, u2);
                for pos in &u1.cross() {
                    if pos == e1 {
                        return Ordering::Less;
                    } else if pos == e2 {
                        return Ordering::Greater;
                    }
                }
                panic!("could not find enemy in cross around us");
            }
            // An immediate candidate is always closer than the path.
            (Candidate::Immediate { .. }, Candidate::Path { .. }) => Ordering::Less,
            (
                Candidate::Path {
                    to: to1,
                    distance: d1,
                    next: n1,
                    ..
                },
                Candidate::Path {
                    to: to2,
                    distance: d2,
                    next: n2,
                    ..
                },
            ) => {
                // shortest distance;
                // reading order for destination;
                // reading order for next step
                d1.cmp(&d2).then(to1.cmp(&to2)).then(n1.cmp(&n2))
            }
            (Candidate::Path { .. }, Candidate::Immediate { .. }) => other.cmp(self),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Path {
    length: usize,
    next: Option<NodeIndex>,
}

impl Path {
    fn new(_nx: NodeIndex) -> Path {
        Path {
            length: 1,
            next: None,
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn next(&self) -> NodeIndex {
        self.next.unwrap()
    }

    fn push(&mut self, nx: NodeIndex) {
        self.length += 1;
        if self.length == 2 {
            assert_eq!(self.next, None);
            self.next = Some(nx);
        }
    }
}

fn path_length(
    cells: &BTreeMap<Position, Cell>,
    from: Position,
    to: Position,
) -> Vec<(usize, Position)> {
    let mut g = Graph::new_undirected();
    let mut map = HashMap::new();
    for (&pos, &cell) in cells {
        if !cell.is_empty() && pos != from {
            continue;
        }
        let cur = *map.entry(pos).or_insert_with(|| g.add_node((pos, cell)));
        for &cross_pos in pos.cross().iter() {
            if !cells[&cross_pos].is_empty() {
                continue;
            }
            let by = *map
                .entry(cross_pos)
                .or_insert_with(|| g.add_node((cross_pos, cells[&cross_pos])));
            g.update_edge(cur, by, ());
        }
    }
    if !map.contains_key(&to) {
        return Vec::new();
    }

    let from_idx = map[&from];
    let to_idx = map[&to];
    let mut bfs = Bfs::new(&g, from_idx);
    let mut paths: HashMap<NodeIndex, HashSet<Path>> = HashMap::default();
    let mut first = HashSet::new();
    first.insert(Path::new(from_idx));
    paths.insert(from_idx, first);
    let mut min_length = astar(&g, from_idx, |i| i == to_idx, |_| 1, |_| 0)
        .map_or(usize::max_value(), |p| p.1.len());
    while let Some(nx) = bfs.next(&g) {
        for neighbor in g.neighbors(nx) {
            let cur = &paths[&nx];
            let mut next = HashSet::with_capacity(cur.len());
            for path in cur {
                let mut path = path.clone();
                path.push(neighbor);
                if neighbor == to_idx {
                    min_length = cmp::min(path.len(), min_length);
                }
                if path.len() <= min_length {
                    next.insert(path);
                }
            }
            paths
                .entry(neighbor)
                .or_insert_with(HashSet::new)
                .extend(next);
        }
    }
    if !paths.contains_key(&to_idx) || paths[&to_idx].is_empty() {
        return Vec::new();
    }
    let min_length = paths[&to_idx].iter().map(|v| v.len()).min().unwrap();
    let mut best_paths = Vec::new();
    for paths in paths.remove(&to_idx) {
        for path in &paths {
            if path.len() > min_length {
                continue;
            }
            best_paths.push(path.to_owned());
        }
    }

    best_paths
        .into_iter()
        .map(|path| (path.len(), g[path.next()].0))
        .collect()
}

impl State {
    fn tick(&mut self) -> bool {
        for cell in self.cells.values_mut() {
            cell.set_visited(false);
        }
        let positions = self.cells.keys().cloned().collect::<Vec<_>>();
        for &pos in &positions {
            let mut cell = self.cells[&pos];
            if cell.visited() {
                continue;
            }
            cell.set_visited(true);
            let enemies = match cell {
                Cell::Goblin(..) => self
                    .cells
                    .iter()
                    .map(|(&k, &v)| (k, v))
                    .filter(|(_, v)| v.is_elf())
                    .collect::<Vec<_>>(),
                Cell::Elf(..) => self
                    .cells
                    .iter()
                    .map(|(&k, &v)| (k, v))
                    .filter(|(_, v)| v.is_goblin())
                    .collect::<Vec<_>>(),
                Cell::Wall | Cell::Empty => continue,
            };
            if enemies.is_empty() {
                return true;
            }
            let enemies = &enemies;
            let mut candidate = None;
            for &(enemy_pos, _) in enemies {
                for &cross_pos in &enemy_pos.cross() {
                    if cross_pos == pos {
                        // already at the enemy
                        let new = Candidate::Immediate {
                            us: pos,
                            enemy: enemy_pos,
                        };
                        if let Some(candidate) = &mut candidate {
                            if *candidate > new {
                                *candidate = new;
                            }
                        } else {
                            candidate = Some(new);
                        }
                    }
                }
            }
            for &(enemy_pos, _) in enemies {
                for cross_pos in &enemy_pos.cross() {
                    let cross_pos: Position = *cross_pos;
                    if !self.cells[&cross_pos].is_empty() {
                        continue;
                    }
                    for (distance, next) in path_length(&self.cells, pos, cross_pos) {
                        let new = Candidate::Path {
                            from: pos,
                            to: cross_pos,
                            enemy: enemy_pos,
                            distance,
                            next,
                        };
                        if let Some(candidate) = &mut candidate {
                            if *candidate > new {
                                *candidate = new;
                            }
                        } else {
                            candidate = Some(new);
                        }
                    }
                }
            }
            let c = if let Some(c) = candidate {
                c
            } else {
                // no known candidate, so skipping this cell
                continue;
            };

            let pos = if let Candidate::Path { next, .. } = c {
                self.cells.insert(pos, Cell::Empty);
                self.cells.insert(next, cell);
                next
            } else {
                pos
            };

            let enemy = pos
                .cross()
                .iter()
                .filter_map(|attack_pos| {
                    self.cells.get(&attack_pos).and_then(|e| {
                        if !cell.is_enemy(*e) {
                            return None;
                        }
                        Some((*attack_pos, *e))
                    })
                })
                .min_by_key(|e| (e.1.health(), e.0));
            let (enemy_pos, enemy_cell) = if let Some(e) = enemy {
                e
            } else {
                continue;
            };
            let enemy_cell = cell.attack(enemy_cell, self.elf_power);
            self.cells
                .insert(enemy_pos, enemy_cell.unwrap_or(Cell::Empty));
            self.cells.insert(pos, cell);
        }

        false
    }

    fn elves_alive(&self) -> usize {
        self.cells.values().filter(|c| c.is_elf()).count()
    }

    fn goblins_alive(&self) -> usize {
        self.cells.values().filter(|c| c.is_goblin()).count()
    }

    fn health(&self) -> Option<usize> {
        let elves_left = self.elves_alive();
        if elves_left == 0 {
            let health = self
                .cells
                .values()
                .filter(|c| c.is_goblin())
                .map(|c| c.health())
                .sum::<usize>();
            return Some(health);
        }
        let goblins_left = self.goblins_alive();
        if goblins_left == 0 {
            let health = self
                .cells
                .values()
                .filter(|c| c.is_elf())
                .map(|c| c.health())
                .sum::<usize>();
            return Some(health);
        }
        None
    }
}

#[generator]
fn generator(input: &str) -> State {
    let mut cells = BTreeMap::new();
    for (line_idx, line) in input.trim().lines().enumerate() {
        for (ch_idx, ch) in line.chars().enumerate() {
            let pos = Position {
                y: line_idx,
                x: ch_idx,
            };
            cells.insert(
                pos,
                match ch {
                    '#' => Cell::Wall,
                    'E' => Cell::Elf(200, false),
                    'G' => Cell::Goblin(200, false),
                    '.' => Cell::Empty,
                    _ => panic!("unknown: {:?}", ch),
                },
            );
        }
    }
    State {
        cells,
        elf_power: 3,
    }
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 27730,
    expect = 208960)]
fn part1(mut state: State) -> usize {
    eprintln!("\n\n");
    let mut i = 0;
    eprintln!("{}", i);
    eprintln!("{:?}", state);
    loop {
        if state.tick() {
            // could not find a target before end
            break;
        }
        i += 1;
        eprintln!("{}", i);
        eprintln!("{:?}", state);
        if let Some(health) = state.health() {
            eprintln!("{} * {}", i, health);
            return i * health;
        }
    }
    eprintln!("{}", i);
    eprintln!("{:?}", state);
    if let Some(health) = state.health() {
        eprintln!("mid-break: {} * {}", i, health);
        return i * health;
    }
    unreachable!()
}

sol_test!(p1_example: part1(generator("
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
")), 27730);

sol_test!(p1_1: part1(generator("
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
")), 36334);

sol_test!(p1_2: part1(generator("
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
")), 39514);

sol_test!(p1_3: part1(generator("
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
")), 28944);

sol_test!(p1_4: part1(generator("
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
")), 18740);

sol_test!(p1_r1: part1(generator("
####
##E#
#GG#
####
")), 13400);

sol_test!(p1_r2: part1(generator("
#####
#GG##
#.###
#..E#
#.#G#
#.E##
#####
")), 13987);

sol_test!(p1_r3: part1(generator("
##########
#.E....G.#
#......###
#.G......#
##########
")), 10325);

sol_test!(p1_r4: part1(generator("
##########
#........#
#......#.#
#E....G#E#
#......#.#
#........#
##########
")), 10804);

sol_test!(p1_r5: part1(generator("
#######
#..E#G#
#.....#
#G#...#
#######
")), 10620);

sol_test!(p1_r6: part1(generator("
#########
#......G#
#G.G...E#
#########
")), 16932);

sol_test!(p1_r7: part1(generator("
######
#.G..#
#...E#
#E...#
######
")), 10234);

sol_test!(p1_r8: part1(generator("
######
#.G..#
##..##
#...E#
#E...#
######
")), 10430);

sol_test!(p1_r9: part1(generator("
########
#.E....#
#......#
#....G.#
#...G..#
#G.....#
########
")), 12744);

sol_test!(p1_r10: part1(generator("
#################
##..............#
##........G.....#
####.....G....###
#....##......####
#...............#
##........GG....#
##.........E..#.#
#####.###...#####
#################
")), 14740);

sol_test!(p1_wall: part1(generator("
################
#.......G......#
#G.............#
#..............#
#....###########
#....###########
#.......EG.....#
################
")), 18468);

sol_test!(p1_move_right: part1(generator("
#######
#.E..G#
#.#####
#G#####
#######
")), 10234);

sol_test!(p1_movement: part1(generator("
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
")), 27828);

sol_test!(p1_11: part1(generator("
######
#..E.#
#....#
#.G..#
#....#
######
")), 134);

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 4988)]
fn part2(state: State) -> usize {
    let elves = state.elves_alive();
    let orig = state.clone();
    'power: for power in 4.. {
        let mut state = orig.clone();
        state.elf_power = power;
        let mut i = 0;
        eprintln!("{}; elf_power = {}", i, power);
        loop {
            if state.tick() {
                // could not find a target before end
                break;
            }
            i += 1;
            eprintln!("{}; elf_power = {}", i, power);
            eprintln!("{:?}", state);
            if state.elves_alive() != elves {
                continue 'power;
            }
            if state.goblins_alive() == 0 {
                return i * state.health().unwrap();
            }
        }
        eprintln!("{}", i);
        eprintln!("{:?}", state);
        if state.elves_alive() != elves {
            continue 'power;
        }
        if state.goblins_alive() == 0 {
            return i * state.health().unwrap();
        }
    }
    unreachable!()
}

static EXAMPLE: &str = "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
";
static INPUT: &str = "
################################
##############..##.#############
###############.##.#############
#####..G#######....#########..##
###......########..########...##
####..#G.#######.G...######...##
####......######...G.#####..####
#####......#####.....#.....G####
###.G..#.#..G..#.G.G....#.######
###..#...#.....G.......#########
###......#....##..........######
###..#.G.#....G...........######
####.G.......G#####......G######
#####....#G..#######.....#######
####........#########...######.#
#######.....#########E..######.#
########....#########.#..#####.#
########....#########E#.E.###..#
########..E.#########......E...#
#######......#######.....E.....#
#######..G....#####.......##.###
#######........G........E.######
#######....................#####
########...............#....####
########.....E..G......##.######
##...#..#.#............##.######
#G.#.#..#.....E........#########
#.................E....#########
###...............#...##########
#####.....#.........#.##########
#####...####.#..#..##..#########
################################
";
