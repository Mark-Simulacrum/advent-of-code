use aoc_macro::{generator, sol_test, solution};
use hashbrown::{HashMap, HashSet};
use petgraph::{
    algo::dijkstra,
    visit::{
        self, EdgeRef, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges, IntoNeighbors, VisitMap,
        Visitable,
    },
};
use std::cell::RefCell;
use std::fmt;

aoc_macro::day!();

type Out = Input;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn up(self) -> Option<Position> {
        let y = self.y.checked_sub(1)?;
        Some(Position { y, x: self.x })
    }
    fn down(self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(self) -> Option<Position> {
        Some(Position {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    fn right(self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn cross(self) -> CrossIter {
        CrossIter { pos: self, idx: 0 }
    }
}

struct CrossIter {
    pos: Position,
    idx: usize,
}

impl Iterator for CrossIter {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        let mut r = None;
        while r.is_none() {
            r = match self.idx {
                0 => self.pos.up(),
                1 => self.pos.left(),
                2 => Some(self.pos.right()),
                3 => Some(self.pos.down()),
                _ => return None,
            };
            self.idx += 1;
        }
        r
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct Input {
    depth: usize,
    target: Position,
    cache: RefCell<HashMap<Position, usize>>,
}

impl Input {
    fn geologic_idx(&self, pos: Position) -> usize {
        if pos.x == 0 && pos.y == 0 {
            return 0;
        }

        if self.target == pos {
            return 0;
        }

        if pos.y == 0 {
            return pos.x * 16807;
        }

        if pos.x == 0 {
            return pos.y * 48271;
        }

        self.erosion_level(pos.left().unwrap()) * self.erosion_level(pos.up().unwrap())
    }

    fn erosion_level(&self, pos: Position) -> usize {
        if let Some(level) = self.cache.borrow().get(&pos) {
            return *level;
        }

        let level = (self.depth + self.geologic_idx(pos)) % 20183;
        self.cache.borrow_mut().insert(pos, level);
        level
    }

    fn ty(&self, pos: Position) -> Type {
        let level = self.erosion_level(pos);
        match level % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => unreachable!("{:?}", level % 3),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Gear {
    Torch,
    Climbing,
    None,
}

impl Gear {
    fn valid(self, ty: Type) -> bool {
        match ty {
            Type::Rocky => self == Gear::Climbing || self == Gear::Torch,
            Type::Wet => self == Gear::Climbing || self == Gear::None,
            Type::Narrow => self == Gear::Torch || self == Gear::None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Person {
    pos: Position,
    gear: Gear,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.gear {
            Gear::Torch => write!(f, "Holding torch @ {:?}", self.pos),
            Gear::Climbing => write!(f, "Holding cgear @ {:?}", self.pos),
            Gear::None => write!(f, "Holding none  @ {:?}", self.pos),
        }
    }
}

impl Person {
    fn valid(&self, input: &Input) -> bool {
        self.gear.valid(input.ty(self.pos))
    }
}

#[generator]
fn generator(input: &str) -> Out {
    let mut depth = None;
    let mut target = None;
    for line in input.trim().lines() {
        if line.starts_with("depth: ") {
            depth = Some(line["depth: ".len()..].parse::<usize>().unwrap());
        } else {
            assert!(line.starts_with("target: "));
            let line = &line["target: ".len()..];
            target = Some(Position {
                x: line[..line.find(',').unwrap()].parse::<usize>().unwrap(),
                y: line[line.find(',').unwrap() + 1..]
                    .parse::<usize>()
                    .unwrap(),
            });
        }
    }
    Input {
        depth: depth.unwrap(),
        target: target.unwrap(),
        cache: Default::default(),
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let pos = Position { x, y };
                let d = match self.ty(pos) {
                    Type::Narrow => '|',
                    Type::Wet => '=',
                    Type::Rocky => '.',
                };
                write!(f, "{}", d)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 114)]
fn part1(input: Out) -> u32 {
    let mut danger = 0;
    for x in 0..=input.target.x {
        for y in 0..=input.target.y {
            let pos = Position { x, y };
            let d = match input.ty(pos) {
                Type::Narrow => 2,
                Type::Wet => 1,
                Type::Rocky => 0,
            };
            danger += d;
        }
    }
    danger
}

#[derive(Copy, Clone)]
struct Map<'a> {
    input: &'a Input,
}

impl<'a> visit::Data for Map<'a> {
    type NodeWeight = ();
    type EdgeWeight = usize;
}

#[derive(Copy, Clone)]
struct MapEdge {
    from: Person,
    to: Person,
    weight: usize,
}

impl EdgeRef for MapEdge {
    type NodeId = Person;
    type EdgeId = ();
    type Weight = usize;

    fn source(&self) -> Person {
        self.from
    }

    fn target(&self) -> Person {
        self.to
    }

    fn weight(&self) -> &usize {
        &self.weight
    }

    fn id(&self) -> Self::EdgeId {
        unimplemented!()
    }
}

impl<'a> IntoEdgeReferences for Map<'a> {
    type EdgeRef = MapEdge;
    type EdgeReferences = std::iter::Empty<Self::EdgeRef>;
    fn edge_references(self) -> Self::EdgeReferences {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Transition {
    Movement,
    SwitchGear,
}

struct Edges<'a> {
    map: Map<'a>,
    node: Person,
    next: Vec<(Person, Transition)>,
}

impl<'a> Iterator for Edges<'a> {
    type Item = MapEdge;
    fn next(&mut self) -> Option<MapEdge> {
        let mut r = None;
        while let Some((person, trans)) = self.next.pop() {
            if person.valid(&self.map.input) {
                r = Some((person, trans));
                break;
            }
        }
        let (person, transition) = r?;
        Some(MapEdge {
            from: self.node,
            to: person,
            weight: match transition {
                Transition::Movement => 1,
                Transition::SwitchGear => 7,
            },
        })
    }
}

impl<'a> IntoEdges for Map<'a> {
    type Edges = Edges<'a>;
    fn edges(self, cur: Person) -> Self::Edges {
        let mut next = Vec::with_capacity(4 + 6); // movement + switching
        for next_pos in cur.pos.cross() {
            next.push((
                Person {
                    pos: next_pos,
                    gear: cur.gear,
                },
                Transition::Movement,
            ));
        }
        let gear_next = match cur.gear {
            Gear::Climbing => [Gear::None, Gear::Torch],
            Gear::Torch => [Gear::None, Gear::Climbing],
            Gear::None => [Gear::Climbing, Gear::Torch],
        };
        for &gear in &gear_next {
            next.push((
                Person {
                    pos: cur.pos,
                    gear: gear,
                },
                Transition::SwitchGear,
            ));
        }
        Edges {
            map: self,
            node: cur,
            next,
        }
    }
}

impl<'a> IntoNeighbors for Map<'a> {
    type Neighbors = std::iter::Empty<Self::NodeId>;
    fn neighbors(self, _: Self::NodeId) -> Self::Neighbors {
        unimplemented!()
    }
}

impl<'a> GraphBase for Map<'a> {
    type EdgeId = ();
    type NodeId = Person;
}

impl<'a> GraphRef for Map<'a> {}

impl VisitMap<Person> for HashSet<Person> {
    fn visit(&mut self, a: Person) -> bool {
        self.insert(a)
    }

    fn is_visited(&self, a: &Person) -> bool {
        self.contains(a)
    }
}

impl<'a> Visitable for Map<'a> {
    type Map = HashSet<Person>;
    fn visit_map(&self) -> Self::Map {
        HashSet::new()
    }
    fn reset_map(&self, map: &mut Self::Map) {
        map.clear();
    }
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 45)]
fn part2(input: Out) -> usize {
    let target = input.target;
    let map = Map { input: &input };
    let start = Person {
        pos: Position { x: 0, y: 0 },
        gear: Gear::Torch,
    };
    let target = Person {
        pos: target,
        gear: Gear::Torch,
    };
    let costs = dijkstra(&map, start, Some(target), |e| *e.weight());
    costs[&target]
}

sol_test!(ge_0: generator(EXAMPLE).ty(Position { x: 0, y: 0 }), Type::Rocky);
sol_test!(ge_1: generator(EXAMPLE).ty(Position { x: 1, y: 0 }), Type::Wet);
sol_test!(ge_2: generator(EXAMPLE).ty(Position { x: 0, y: 1 }), Type::Rocky);
sol_test!(ge_3: generator(EXAMPLE).ty(Position { x: 1, y: 1 }), Type::Narrow);
sol_test!(ge_4: generator(EXAMPLE).ty(Position { x: 10, y: 10 }), Type::Rocky);

static EXAMPLE: &str = "
depth: 510
target: 10,10
";
static INPUT: &str = "
depth: 11394
target: 7,701
";
