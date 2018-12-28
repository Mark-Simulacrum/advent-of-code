use aoc_macro::{generator, solution, sol_test};
use std::fmt;
use generational_arena::{Arena, Index};

aoc_macro::day!();

type Out<'a> = (u32, u32);

#[generator]
fn generator(input: Out) -> Out {
    input
}

struct Node {
    value: u32,
    prev: Index,
    next: Index,
}

struct Circle {
    arena: Arena<Node>,
    zero: Index,
    cursor: Index,
}

impl Circle {
    fn with_capacity(cap: usize) -> Circle {
        // This is our work-around for creating a node with indices that point to itself
        let mut fake = Arena::new();
        let fake = fake.insert(());

        let mut a = Arena::with_capacity(cap);
        let cursor = a.insert(Node { value: 0, prev: fake, next: fake });
        a[cursor].prev = cursor;
        a[cursor].next = cursor;
        Circle {
            arena: a,
            zero: cursor,
            cursor,
        }
    }

    fn forward(&mut self, n: usize) {
        for _ in 0..n {
            self.cursor = self.arena[self.cursor].next;
        }
    }

    fn backward(&mut self, n: usize) {
        for _ in 0..n {
            self.cursor = self.arena[self.cursor].prev;
        }
    }

    fn remove(&mut self) -> u32 {
        let node = self.arena.remove(self.cursor).expect("node to exist");
        self.arena[node.prev].next = node.next;
        self.arena[node.next].prev = node.prev;
        self.cursor = node.next;
        assert!(self.arena.contains(self.cursor));
        node.value
    }

    fn insert(&mut self, v: u32) {
        let prev = self.arena[self.cursor].prev;
        let next = self.cursor;
        let node = self.arena.insert(Node {
            value: v,
            prev,
            next,
        });
        self.arena[prev].next = node;
        self.arena[next].prev = node;
        self.cursor = node;
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cur = self.zero;
        loop {
            let node = &self.arena[cur];
            write!(f, "{} ", node.value)?;
            cur = node.next;
            if cur == self.zero {
                break;
            }
        }
        Ok(())
    }
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 32,
    expect = 428690)]
fn part1((players, marbles): Out) -> u32 {
    let mut elfs: Vec<u32> = vec![0; players as usize];
    let mut circle = Circle::with_capacity(marbles as usize);
    let mut next_marble: u32 = 1;
    'outer: loop {
        for elf in &mut elfs {
            if next_marble % 23 == 0 {
                *elf += next_marble;
                circle.backward(7);
                let marble = circle.remove();
                *elf += marble;
            } else {
                circle.forward(2);
                circle.insert(next_marble);
            }
            next_marble += 1;

            if next_marble > marbles {
                break 'outer;
            }
        }
    }
    elfs.into_iter().max().unwrap()
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 22563)]
fn part2((players, marbles): Out) -> u32 {
    part1((players, marbles * 100))
}

sol_test!(p1: part1((10, 1618)), 8317);
sol_test!(p2: part1((13, 7999)), 146373);
sol_test!(p3: part1((17, 1104)), 2764);
sol_test!(p4: part1((21, 6111)), 54718);
sol_test!(p5: part1((30, 5807)), 37305);

static EXAMPLE: Out = (9, 25);
static INPUT: Out = (405, 71700);
