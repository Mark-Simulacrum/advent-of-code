use aoc_macro::{generator, solution};
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use petgraph::stable_graph::StableGraph;
use fnv::FnvHashMap;
use petgraph::visit::IntoNodeReferences;

type Out = StableGraph<char, ()>;

fn externals<'a>(g: &'a Out) -> impl Iterator<Item=(char, NodeIndex)> + 'a {
    g
        .node_references()
        .filter_map(move |(node, ch)| {
            if g.edges_directed(node, Direction::Incoming).count() == 0 {
                Some((*ch, node))
            } else {
                None
            }
        })
}

#[generator]
fn generator(input: &str) -> Out {
    let mut g = StableGraph::new();
    let mut nodes = FnvHashMap::default();
    for l in input.trim().lines() {
        let finish = l.chars().nth(5).unwrap();
        let before = l.chars().nth("Step A must be finished before step ".len()).unwrap();

        let finish = *nodes.entry(finish).or_insert_with(|| g.add_node(finish));
        let before = *nodes.entry(before).or_insert_with(|| g.add_node(before));
        g.add_edge(finish, before, ());
    }
    g
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = "CABDFE",
    expect = "BGKDMJCNEQRSTUZWHYLPAFIVXO")]
fn part1(mut input: Out) -> String {
    let mut out = String::new();
    let mut root = externals(&input).min();
    while let Some(r) = root {
        out.push(r.0);
        input.remove_node(r.1);
        root = externals(&input).min();
    }
    out
}

#[derive(Debug)]
struct Worker {
    // None if not working
    time_left: u8,
    node: NodeIndex,
}

impl Worker {
    fn tick(mut self, g: &mut Out) -> Option<Worker> {
        self.time_left -= 1;
        if self.time_left == 0 {
            g.remove_node(self.node);
            None
        } else {
            Some(self)
        }
    }

    fn make(g: &Out, idx: NodeIndex, example: bool) -> Worker {
        let delta = if example { 0 } else { 60 };
        Worker {
            time_left: g[idx] as u8 - b'A' + 1 + delta,
            node: idx,
        }
    }
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 15,
    expect = 941)]
fn part2(mut input: Out, example: bool) -> usize {
    let mut roots = externals(&input).collect::<Vec<_>>();
    let mut time = 0;
    let mut workers = Vec::new();
    let mut started = fnv::FnvHashSet::default();
    while !roots.is_empty() {
        roots.sort();
        loop {
            // We can have at most 2 workers for the example
            if example && workers.len() == 2 {
                break;
            }
            // We can have at most 5 workers for the actual code
            if !example && workers.len() == 5 {
                break;
            }
            // Need new roots
            if roots.is_empty() {
                break;
            }
            let (c, idx) = roots.remove(0);
            if started.insert(c) {
                let w = Worker::make(&input, idx, example);
                workers.push(w);
            }
        }
        workers = workers.into_iter().filter_map(|w| w.tick(&mut input)).collect();
        roots = externals(&input).collect::<Vec<_>>();
        time += 1;
    }
    time
}

static EXAMPLE: &str = "
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

static INPUT: &str = "
Step Q must be finished before step I can begin.
Step B must be finished before step M can begin.
Step R must be finished before step F can begin.
Step G must be finished before step S can begin.
Step M must be finished before step A can begin.
Step Z must be finished before step W can begin.
Step J must be finished before step C can begin.
Step K must be finished before step O can begin.
Step C must be finished before step I can begin.
Step Y must be finished before step L can begin.
Step N must be finished before step P can begin.
Step S must be finished before step X can begin.
Step E must be finished before step U can begin.
Step U must be finished before step V can begin.
Step D must be finished before step F can begin.
Step W must be finished before step H can begin.
Step T must be finished before step I can begin.
Step H must be finished before step V can begin.
Step L must be finished before step O can begin.
Step P must be finished before step A can begin.
Step A must be finished before step I can begin.
Step F must be finished before step O can begin.
Step V must be finished before step X can begin.
Step I must be finished before step O can begin.
Step X must be finished before step O can begin.
Step F must be finished before step V can begin.
Step L must be finished before step P can begin.
Step Y must be finished before step P can begin.
Step Y must be finished before step X can begin.
Step Y must be finished before step O can begin.
Step D must be finished before step A can begin.
Step T must be finished before step F can begin.
Step W must be finished before step X can begin.
Step R must be finished before step A can begin.
Step E must be finished before step F can begin.
Step H must be finished before step I can begin.
Step K must be finished before step Y can begin.
Step W must be finished before step P can begin.
Step V must be finished before step O can begin.
Step N must be finished before step E can begin.
Step L must be finished before step I can begin.
Step B must be finished before step G can begin.
Step D must be finished before step T can begin.
Step J must be finished before step L can begin.
Step M must be finished before step Y can begin.
Step T must be finished before step A can begin.
Step K must be finished before step D can begin.
Step H must be finished before step P can begin.
Step P must be finished before step I can begin.
Step T must be finished before step L can begin.
Step J must be finished before step N can begin.
Step U must be finished before step F can begin.
Step U must be finished before step I can begin.
Step A must be finished before step F can begin.
Step U must be finished before step P can begin.
Step R must be finished before step H can begin.
Step G must be finished before step V can begin.
Step P must be finished before step F can begin.
Step B must be finished before step D can begin.
Step U must be finished before step X can begin.
Step K must be finished before step A can begin.
Step G must be finished before step D can begin.
Step N must be finished before step U can begin.
Step U must be finished before step L can begin.
Step M must be finished before step J can begin.
Step I must be finished before step X can begin.
Step H must be finished before step L can begin.
Step M must be finished before step S can begin.
Step E must be finished before step O can begin.
Step Q must be finished before step F can begin.
Step A must be finished before step O can begin.
Step T must be finished before step P can begin.
Step F must be finished before step X can begin.
Step D must be finished before step P can begin.
Step A must be finished before step X can begin.
Step G must be finished before step Z can begin.
Step W must be finished before step F can begin.
Step Q must be finished before step X can begin.
Step C must be finished before step V can begin.
Step L must be finished before step V can begin.
Step E must be finished before step L can begin.
Step B must be finished before step X can begin.
Step M must be finished before step V can begin.
Step F must be finished before step I can begin.
Step P must be finished before step X can begin.
Step C must be finished before step A can begin.
Step Z must be finished before step H can begin.
Step Q must be finished before step S can begin.
Step G must be finished before step X can begin.
Step T must be finished before step O can begin.
Step P must be finished before step O can begin.
Step T must be finished before step V can begin.
Step N must be finished before step V can begin.
Step Z must be finished before step X can begin.
Step L must be finished before step X can begin.
Step Z must be finished before step Y can begin.
Step N must be finished before step T can begin.
Step S must be finished before step T can begin.
Step G must be finished before step K can begin.
Step T must be finished before step X can begin.
Step R must be finished before step X can begin.
";
