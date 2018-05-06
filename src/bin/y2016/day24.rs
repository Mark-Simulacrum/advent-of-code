use std::cmp;
use std::collections::HashMap;

use permutohedron::Heap;
use petgraph::{algo, graph::NodeIndex, Graph, Undirected};

type Maze = Graph<(usize, usize), (), Undirected>;

fn parse(
    s: &str,
) -> (
    Graph<(usize, usize), (), Undirected>,
    HashMap<(usize, usize), NodeIndex>,
    HashMap<usize, NodeIndex>,
) {
    let mut map = HashMap::new();
    let mut landmarks = HashMap::new();
    let mut graph = Graph::new_undirected();
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' || ch.is_digit(10) {
                let idx = graph.add_node((x, y));
                map.insert((x, y), idx);

                if let Some(digit) = ch.to_digit(10) {
                    landmarks.insert(digit as usize, idx);
                }

                if let Some(x1) = x.checked_sub(1) {
                    if let Some(before) = map.get(&(x1, y)) {
                        graph.add_edge(idx, *before, ());
                    }
                }

                if let Some(y1) = y.checked_sub(1) {
                    if let Some(before) = map.get(&(x, y1)) {
                        graph.add_edge(idx, *before, ());
                    }
                }
            }
        }
    }
    (graph, map, landmarks)
}

fn shortest_distance<F: FnMut(NodeIndex) -> bool>(
    graph: &Maze,
    start: NodeIndex,
    goal: F,
) -> (usize, NodeIndex) {
    let (distance, path) = algo::astar(graph, start, goal, |_| 1, |_| 0).unwrap();
    (distance, *path.last().unwrap())
}

pub fn part1(s: &str) -> usize {
    let (graph, _map, landmarks) = parse(s);
    let mut v = landmarks
        .values()
        .cloned()
        .filter(|v| *v != landmarks[&0])
        .collect::<Vec<_>>();

    let heap = Heap::new(&mut v);
    let mut minimum = usize::max_value();
    'p: for mut order in heap {
        let mut total_distance = 0;
        let mut previous = landmarks[&0];
        while let Some(next) = order.pop() {
            let (dist, to_node) = shortest_distance(&graph, previous, |n| n == next);
            total_distance += dist;
            if total_distance >= minimum {
                continue 'p;
            }
            previous = to_node;
        }
        minimum = cmp::min(minimum, total_distance);
    }
    minimum
}

pub fn part2(s: &str) -> usize {
    let (graph, _map, landmarks) = parse(s);
    let mut v = landmarks
        .values()
        .cloned()
        .filter(|v| *v != landmarks[&0])
        .collect::<Vec<_>>();

    let heap = Heap::new(&mut v);
    let mut minimum = usize::max_value();
    'p: for mut order in heap {
        let mut total_distance = 0;
        let mut previous = landmarks[&0];
        while let Some(next) = order.pop() {
            let (dist, to_node) = shortest_distance(&graph, previous, |n| n == next);
            total_distance += dist;
            if total_distance >= minimum {
                continue 'p;
            }
            previous = to_node;
        }
        total_distance += shortest_distance(&graph, previous, |n| n == landmarks[&0]).0;
        minimum = cmp::min(minimum, total_distance);
    }
    minimum
}

pub static INPUT: &str = include_str!("day24.input");
