use aoc_macro::{generator, solution};

type Out = (Set, Patterns);

#[generator]
fn generator((input, patterns): (&'static str, &'static str)) -> Out {
    let input = input.trim();
    let patterns = patterns.trim().lines().map(|line| {
        let mut s = line.split(" => ");
        let pattern = s.next().unwrap().as_bytes();
        let p = [
            pattern[0] == b'#',
            pattern[1] == b'#',
            pattern[2] == b'#',
            pattern[3] == b'#',
            pattern[4] == b'#',
        ];
        let pattern =
            ((p[0] as u8) << 4) |
            ((p[1] as u8) << 3) |
            ((p[2] as u8) << 2) |
            ((p[3] as u8) << 1) |
            ((p[4] as u8) << 0);
        let out = s.next().unwrap().as_bytes()[0] == b'#';
        (pattern, out)
    }).collect::<Vec<_>>();

    let input = Set::from(input.as_bytes().iter().enumerate().filter_map(|(idx, b)| {
        if *b == b'#' {
            Some(idx as isize)
        } else {
            None
        }
    }));

    (input, Patterns::new(patterns))
}

mod set {
    use super::{Window};
    use std::iter::once;
    use std::fmt;

    #[derive(Clone)]
    pub struct Set(hashbrown::HashSet<isize>);

    impl fmt::Debug for Set {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl fmt::Display for Set {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut v = self.0.iter().collect::<Vec<_>>();
            v.sort();
            let mut prev = **v.iter().next().unwrap();
            write!(f, "({}) ", prev)?;
            for &&idx in &v {
                for _ in prev..(idx - 1) {
                    write!(f, ".")?;
                }
                write!(f, "#")?;
                prev = idx;
            }
            Ok(())
        }
    }

    impl<I> From<I> for Set
        where I: IntoIterator<Item=isize>,
    {
        fn from(v: I) -> Self {
            Set(v.into_iter().collect())
        }
    }

    impl Set {
        pub fn with_capacity(cap: usize) -> Set {
            Set(hashbrown::HashSet::with_capacity(cap))
        }

        pub fn insert(&mut self, v: isize) {
            self.0.insert(v);
        }

        pub fn get(&self, v: isize) -> bool {
            self.0.contains(&v)
        }

        pub fn iter<'a>(&'a self) -> impl Iterator<Item=Window> + 'a {
            self.0.iter().flat_map(move |start| {
                let start = *start;
                let b = [
                    self.get(start - 4), // a
                    self.get(start - 3), // ab
                    self.get(start - 2), // abc   *
                    self.get(start - 1), // abcd  *
                    self.get(start),     // abcde *
                    self.get(start + 1), //  bcde *
                    self.get(start + 2), //   cde *
                    self.get(start + 3), //    de
                    self.get(start + 4), //     e
                ];

                once(Window::new(start - 2, &b[0..5]))
                    .chain(once(Window::new(start - 1, &b[1..6])))
                    .chain(once(Window::new(start - 0, &b[2..7])))
                    .chain(once(Window::new(start + 1, &b[3..8])))
                    .chain(once(Window::new(start + 2, &b[4..9])))
            })
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn count(&self) -> isize {
            self.0.iter().sum::<isize>()
        }

        pub fn clear(&mut self) {
            self.0.clear()
        }
    }
}

use self::set::Set;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Window {
    center: isize,
    range: u8,
}

impl Window {
    fn new(center: isize, v: &[bool]) -> Self {
        Window {
            center,
            range:
            ((v[0] as u8) << 4) |
            ((v[1] as u8) << 3) |
            ((v[2] as u8) << 2) |
            ((v[3] as u8) << 1) |
            ((v[4] as u8) << 0)
        }
    }

    fn to_usize(self) -> usize {
        self.range as usize
    }
}

struct Patterns {
    map: Vec<bool>,
}

impl Patterns {
    fn new(v: Vec<(u8, bool)>) -> Self {
        let map = v.iter().cloned().collect::<fnv::FnvHashMap<_, _>>();
        let mut vec_map = Vec::new();
        for idx in 0..=0b11111 { // max window
            match map.get(&idx) {
                Some(true) => vec_map.push(true),
                Some(false) => vec_map.push(false),
                None => vec_map.push(false),
            }
        }
        Patterns {
            map: vec_map,
        }
    }
}

struct Cycler {
    current: Set,
    next: Set,
    patterns: Patterns,
}

impl Cycler {
    fn new(input: Set, patterns: Patterns) -> Cycler {
        Cycler {
            next: Set::with_capacity(input.len()),
            current: input,
            patterns,
        }
    }

    fn cycle(&mut self) {
        self.next.clear();
        for window in self.current.iter() {
            match self.patterns.map[window.to_usize()] {
                true => self.next.insert(window.center),
                false => {}
            }
        }
        std::mem::swap(&mut self.current, &mut self.next)
    }

    fn view(&self) -> &Set {
        &self.current
    }
}

fn iterate((input, patterns): Out, cycles: usize) -> isize {
    let mut cycler = Cycler::new(input, patterns);
    for i in 1..=cycles {
        cycler.cycle();
        if i % 1_000_000 == 0 {
            eprintln!("{} / {}: {:.2}%; {}",
                i, cycles, i as f64 / cycles as f64 * 100.0,
                cycler.view());
        }
    }
    cycler.view().count()
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 325,
    expect = 2140)]
fn part1((input, patterns): Out) -> isize {
    iterate((input, patterns), 20)
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 0,
    expect = 1900000000384)]
fn part2(input: Out) -> isize {
    iterate(input, 50000000000)
}

static EXAMPLE: (&str, &str) = (
"#..#.#..##......###...###..............",
"
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
);

static INPUT: (&str, &str) = (
"##.#.#.##..#....######..#..#...#.#..#.#.#..###.#.#.#..#..###.##.#..#.##.##.#.####..##...##..#..##.#.",
"
...## => #
#.#.# => #
.###. => #
#.#.. => .
.#..# => #
#..#. => #
..##. => .
....# => .
#.... => .
###.. => #
.#### => #
###.# => .
#..## => #
..... => .
##.## => #
####. => .
##.#. => .
#...# => .
##### => .
..#.. => .
.#.#. => .
#.### => .
.##.# => .
..#.# => .
.#.## => #
...#. => .
##... => #
##..# => #
.##.. => .
.#... => #
#.##. => #
..### => .
");

/*
##### => .
####. => .
###.# => .
###.. => #
##.## => #
##.#. => .
##..# => #
##... => #
#.### => .
#.##. => #
#.#.# => #
#.#.. => .
#..## => #
#..#. => #
#...# => .
#.... => .
.#### => #
.###. => #
.##.# => .
.##.. => .
.#.## => #
.#.#. => .
.#..# => #
.#... => #
..### => .
..##. => .
..#.# => .
..#.. => .
...## => #
...#. => .
....# => .
..... => .
*/
