use aoc_macro::{generator, solution};

aoc_macro::day!();

type Out<'a> = Vec<u8>;

#[generator]
fn generator(input: &str) -> Out {
    input.trim().split(' ')
        .map(|v| v.parse::<u8>().expect(v))
        .collect()

}

#[derive(Copy, Clone)]
struct Buffer<'a> {
    v: &'a [u8],
}

impl<'a> Buffer<'a> {
    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    fn read(&mut self) -> u8 {
        let value = self.v[0];
        *self = Buffer { v: &self.v[1..] };
        value
    }
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    fn parse(v: &mut Buffer) -> Node {
        let children = v.read();
        let metadata = v.read();
        let children = (0..children).map(|_| Node::parse(v)).collect();
        let metadata = (0..metadata).map(|_| v.read()).collect();
        Node {
            children,
            metadata,
        }
    }

    fn all(&self) -> Vec<&Node> {
        std::iter::once(self)
            .chain(self.children.iter().flat_map(|c| c.all()))
            .collect()
    }

    fn value(&self) -> u32 {
        let mut sum = 0;
        if self.children.is_empty() {
            sum += self.metadata.iter().map(|v| *v as u32).sum::<u32>();
        } else {
            for idx in &self.metadata {
                let idx = idx - 1;
                sum += self.children.get(idx as usize).map(|c| c.value()).unwrap_or(0);
            }
        }
        sum
    }
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 138,
    expect = 47244)]
fn part1(input: Out) -> u32 {
    let mut input = Buffer { v: &input[..] };
    let mut sum = 0;
    while !input.is_empty() {
        let node = Node::parse(&mut input);
        sum += node.all().iter().flat_map(|n| n.metadata.iter()).map(|v| *v as u32).sum::<u32>();
    }
    sum
}


#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 66,
    expect = 17267)]
fn part2(input: Out) -> u32 {
    let mut input = Buffer { v: &input[..] };
    let node = Node::parse(&mut input);
    node.value()
}

static EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
static INPUT: &str = include_str!("day08.input");
