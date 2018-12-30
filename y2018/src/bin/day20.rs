use aoc_macro::{generator, sol_test, solution};
use hashbrown::HashMap;
use regex_syntax::hir::{self, Hir, HirKind};
use regex_syntax::ParserBuilder;
use std::fmt;

aoc_macro::day!();

type Out = Hir;

#[generator]
fn generator(input: &str) -> Out {
    let input = input.trim();
    let mut parser = ParserBuilder::new();
    parser.nest_limit(input.len() as u32);
    let mut parser = parser.build();
    parser.parse(input).unwrap()
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 31,
    expect = 4344)]
fn part1(hir: Out) -> usize {
    let mut map = HashMap::new();

    map.insert(Position::default(), 0);
    descend(&mut map, &hir, Position::default());

    map.values().cloned().max().unwrap()
}

fn descend(map: &mut HashMap<Position, usize>, hir: &Hir, pos: Position) -> Position {
    match hir.kind() {
        HirKind::Anchor(_) | HirKind::Empty => pos,
        HirKind::Class(_)
        | HirKind::WordBoundary(_)
        | HirKind::Repetition(_)
        | HirKind::Literal(hir::Literal::Byte(_)) => {
            unreachable!("regex should not have {:?}", hir.kind())
        }
        HirKind::Literal(hir::Literal::Unicode(ch)) => {
            let dist_to_here = map[&pos] + 1;
            let next_pos = pos.apply(*ch);
            let entry = map.entry(next_pos).or_insert(dist_to_here);
            *entry = std::cmp::min(*entry, dist_to_here);
            next_pos
        }
        HirKind::Group(g) => descend(map, &g.hir, pos),
        HirKind::Concat(hirs) => {
            let mut next_pos = pos;
            for hir in hirs {
                next_pos = descend(map, hir, next_pos);
            }
            next_pos
        }
        HirKind::Alternation(hirs) => {
            for hir in hirs {
                descend(map, hir, pos);
            }
            pos
        }
    }
}

sol_test!(p1_1: part1(generator("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$")), 23);
sol_test!(p1_2: part1(generator("^WNE$")), 3);
sol_test!(p1_3: part1(generator("^ENWWW(NEEE|SSE(EE|N))$")), 10);
sol_test!(p1_4: part1(generator("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")), 18);

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0,
    expect = 8809)]
fn part2(hir: Out) -> usize {
    let mut map = HashMap::new();

    map.insert(Position::default(), 0);
    descend(&mut map, &hir, Position::default());

    map.values().cloned().filter(|v| *v >= 1000).count()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Position {
    y: isize,
    x: isize,
}

impl Position {
    fn apply(self, ch: char) -> Position {
        match ch {
            'N' => Position {
                y: self.y - 1,
                x: self.x,
            },
            'E' => Position {
                x: self.x + 1,
                y: self.y,
            },
            'W' => Position {
                x: self.x - 1,
                y: self.y,
            },
            'S' => Position {
                y: self.y + 1,
                x: self.x,
            },
            _ => unreachable!("unexpected character: {:?}", ch),
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

static EXAMPLE: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
static INPUT: &str = include_str!("day20.input");
