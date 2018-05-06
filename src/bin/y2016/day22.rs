use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;

use petgraph::graph::Graph;

pub fn part1(s: &str) -> usize {
    let nodes = parse(s);

    let mut viable = 0;
    for a in nodes.iter() {
        if a.used == 0 {
            continue;
        }
        for b in nodes.iter() {
            if a == b {
                continue;
            }
            if a.used <= b.available {
                viable += 1;
            }
        }
    }

    viable
}

pub fn part2(s: &str) -> usize {
    let mut nodes = BTreeMap::new();
    let mut graph_nodes = HashMap::new();
    let mut graph = Graph::new_undirected();
    for node in parse(s) {
        nodes.entry(node.y).or_insert_with(Vec::new).push(node);
        graph_nodes.insert((node.x, node.y), graph.add_node(node));
    }
    for x in 0usize..=37 {
        for y in 0usize..=25 {
            if let Some(x1) = x.checked_sub(1) {
                if !(x == 1 && y == 13) {
                    graph.add_edge(graph_nodes[&(x, y)], graph_nodes[&(x1, y)], ());
                }
            }
            if let Some(y1) = y.checked_sub(1) {
                if x == 0 || y != 14 {
                    graph.add_edge(graph_nodes[&(x, y)], graph_nodes[&(x, y1)], ());
                }
            }
        }
    }
    let (distance_to_data, _) = ::petgraph::algo::astar(
        &graph,
        graph_nodes[&(17, 22)],
        |n| n == graph_nodes[&(37, 0)],
        |_| 1,
        |_| 0,
    ).unwrap();
    let (distance_to_goal, _) = ::petgraph::algo::astar(
        &graph,
        graph_nodes[&(37, 0)],
        |n| n == graph_nodes[&(0, 0)],
        |_| 1,
        |_| 0,
    ).unwrap();
    // goal * 5 because we have to dance around it in 5-step increments along the straight path
    // goal - 1 because we don't need to actually hit it; we're going to stop a bit before it - from
    // there we can shift once to get into the position (something accounted by distance_to_data,
    // which is one away from the actual distance).
    //
    // Realistically, though, the "better" way to do this is to do so by hand. The puzzle we're
    // solving is fairly simple, and can be done trivially by hand, avoiding the graph manipulations
    // and heuristic based approaches we apply here.
    distance_to_data + ((distance_to_goal - 1) * 5)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    available: usize,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:02}, {:02}) s: {}, u: {}, a: {}",
            self.x, self.y, self.size, self.used, self.available
        )
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.used == 0 {
            return write!(f, "_");
        }

        if self.used > 100 {
            return write!(f, "#");
        }

        if self.x == 0 && self.y == 0 {
            return write!(f, "T");
        }

        if self.x == 37 && self.y == 0 {
            return write!(f, "G");
        }

        write!(f, ".")
    }
}

fn parse(s: &str) -> Vec<Node> {
    s.trim()
        .lines()
        .map(|line| {
            let dash1 = "/dev/grid/node-".len();
            let dash1 = dash1 + 1 + line[dash1 + 1..].find('-').unwrap();
            let x = line["/dev/grid/node-x".len()..dash1].parse().unwrap();
            let space1 = line.find(' ').unwrap();
            let y = line[line.find('y').unwrap() + 1..space1].parse().unwrap();
            let without_prefix = &line["/dev/grid/node-x##-y##".len()..];
            let first_t = without_prefix.find('T').unwrap();
            let second_t = 1 + first_t + without_prefix[first_t + 1..].find('T').unwrap();
            let third_t = 1 + second_t + without_prefix[second_t + 1..].find('T').unwrap();
            let size = without_prefix[..first_t].trim_left().parse().unwrap();
            let used = without_prefix[first_t + 1..second_t]
                .trim()
                .parse()
                .unwrap();
            let available = without_prefix[second_t + 1..third_t]
                .trim()
                .parse()
                .unwrap();
            Node {
                x,
                y,
                size,
                used,
                available,
            }
        })
        .collect()
}

pub static INPUT: &str = include_str!("day22.input");
