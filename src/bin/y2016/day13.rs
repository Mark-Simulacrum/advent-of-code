use petgraph::algo;
use petgraph::visit::{Data, EdgeRef, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges,
                      IntoNeighbors, VisitMap, Visitable};
use smallvec::{self, SmallVec};

use advent_of_code::BitVec;
use advent_of_code::VecLike;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Point(u32, u32);

impl Point {
    fn index(self) -> u64 {
        assert_eq!(self.0 as u16 as u32, self.0);
        assert_eq!(self.1 as u16 as u32, self.1);
        ((self.0 as u64) << 16) | (self.1 as u64)
    }

    fn is_wall(self, n: u32) -> bool {
        let Point(x, y) = self;
        let num = x * x + 3 * x + 2 * x * y + y + y * y + n;
        num.count_ones() % 2 != 0
    }

    fn left(self) -> Option<Point> {
        self.0.checked_sub(1).map(|x| Point(x, self.1))
    }

    fn right(self) -> Point {
        Point(self.0 + 1, self.1)
    }

    fn up(self) -> Option<Point> {
        self.1.checked_sub(1).map(|x| Point(self.0, x))
    }

    fn down(self) -> Point {
        Point(self.0, self.1 + 1)
    }
}

#[derive(Copy, Clone, Debug)]
struct Maze(u32);

impl Maze {
    fn distance(&self, a: Point, b: Point) -> u32 {
        algo::dijkstra(self, a, Some(b), |_| 1)[&b]
    }
}

impl GraphBase for Maze {
    type EdgeId = MazeEdge;
    type NodeId = Point;
}

impl GraphRef for Maze {}

impl Data for Maze {
    type NodeWeight = ();
    type EdgeWeight = ();
}

struct VisitSet(BitVec);

impl VisitMap<Point> for VisitSet {
    fn visit(&mut self, a: Point) -> bool {
        let has = self.0.get(a.index() as usize);
        self.0.set(a.index() as usize, true);
        !has
    }

    fn is_visited(&self, a: &Point) -> bool {
        self.0.get(a.index() as usize)
    }
}

impl Visitable for Maze {
    type Map = VisitSet;
    fn visit_map(&self) -> Self::Map {
        VisitSet(BitVec::new())
    }

    fn reset_map(&self, map: &mut Self::Map) {
        map.0.clear()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct MazeEdge {
    from: Point,
    to: Point,
}

impl EdgeRef for MazeEdge {
    type NodeId = Point;
    type EdgeId = MazeEdge;
    type Weight = ();
    fn source(&self) -> Self::NodeId {
        self.from
    }
    fn target(&self) -> Self::NodeId {
        self.to
    }
    fn weight(&self) -> &Self::Weight {
        &()
    }
    fn id(&self) -> Self::EdgeId {
        *self
    }
}

impl<'a> IntoEdgeReferences for &'a Maze {
    type EdgeRef = MazeEdge;
    type EdgeReferences = ::std::iter::Empty<MazeEdge>;
    fn edge_references(self) -> Self::EdgeReferences {
        unimplemented!()
    }
}

impl IntoNeighbors for Maze {
    type Neighbors = smallvec::IntoIter<[Point; 4]>;
    fn neighbors(self, a: Point) -> Self::Neighbors {
        let mut points = SmallVec::new();
        if let Some(pt) = a.left() {
            if !pt.is_wall(self.0) {
                points.push(pt);
            }
        }
        if let Some(pt) = a.up() {
            if !pt.is_wall(self.0) {
                points.push(pt);
            }
        }
        {
            let pt = a.right();
            if !pt.is_wall(self.0) {
                points.push(pt);
            }
        }
        {
            let pt = a.down();
            if !pt.is_wall(self.0) {
                points.push(pt);
            }
        }
        points.into_iter()
    }
}

impl<'a> IntoEdges for &'a Maze {
    type Edges = smallvec::IntoIter<[MazeEdge; 4]>;
    fn edges(self, a: Point) -> Self::Edges {
        self.neighbors(a)
            .map(|n| MazeEdge { from: a, to: n })
            .collect::<SmallVec<[_; 4]>>()
            .into_iter()
    }
}

pub fn part1(input: u32) -> u32 {
    Maze(input).distance(Point(1, 1), Point(31, 39))
}

pub fn part2(input: u32) -> u32 {
    let maze = Maze(input);
    let mut within_distance = 0;
    let mut stack = Vec::new();
    stack.push(Point(1, 1));
    let mut visited = VisitSet(BitVec::new());
    while let Some(nx) = stack.pop() {
        if visited.visit(nx) {
            let d = maze.distance(Point(1, 1), nx);
            if d <= 50 {
                within_distance += 1;
                stack.extend(maze.neighbors(nx));
            }
        }
    }
    within_distance
}

#[test]
fn part1_1() {
    assert_eq!(Maze(10).distance(Point(1, 1), Point(7, 4)), 11);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 82);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 138);
}

pub const INPUT: u32 = 1362;
