#[derive(Copy, Clone, Debug)]
struct Layer {
    depth: u8,
    range: u8,
}

#[derive(Clone)]
struct Layers {
    scanner: u64,
    layers: Vec<Layer>,
}

impl Layers {
    fn parse(s: &str) -> Layers {
        let mut layers = Vec::new();
        for l in s.trim().lines() {
            let mut items = l.split(": ");
            let depth = items.next().unwrap().parse::<u8>().unwrap();
            let range = items.next().unwrap().parse::<u8>().unwrap();
            while depth as usize != layers.len() {
                let l = layers.len();
                layers.push(Layer { depth: l as u8, range: 0 });
            }
            layers.push(Layer {
                depth,
                range: range.saturating_sub(1),
            });
        }
        Layers { layers: layers, scanner: 0 }
    }

    fn run(&mut self) -> usize {
        let mut severity = 0;
        for layer in self.layers.iter() {
            // multiply range by 2 for trip there and back again
            let range = (layer.range as u64) * 2;
            if range != 0 && self.scanner % range == 0 {
                severity += layer.depth as usize * (layer.range as usize + 1);
            }
            self.scanner += 1;
        }
        severity
    }

}

pub fn part1(s: &str) -> usize {
    Layers::parse(s).run()
}

pub fn part2(s: &str) -> u64 {
    let layers = Layers::parse(s);
    let mut ranges = layers.layers.iter()
        .enumerate()
        .filter(|&(_, l)| l.range != 0)
        .map(|(i, l)| (i as u64, (l.range as u64) * 2))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|&(_, r)| r);
    for delay in 0u64.. {
        let mut caught = false;
        for &(i, range) in &ranges {
            if (delay + i) % range == 0 {
                caught = true;
                break;
            }
        }
        if !caught {
            return delay;
        }
    }
    unreachable!()
}


#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 24);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 1900);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE), 10);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 3966414);
}


#[cfg(test)]
static EXAMPLE: &str = "
0: 3
1: 2
4: 4
6: 4
";

pub static INPUT: &str = "
0: 3
1: 2
2: 4
4: 4
6: 5
8: 6
10: 6
12: 6
14: 6
16: 8
18: 8
20: 8
22: 8
24: 10
26: 8
28: 8
30: 12
32: 14
34: 12
36: 10
38: 12
40: 12
42: 9
44: 12
46: 12
48: 12
50: 12
52: 14
54: 14
56: 14
58: 12
60: 14
62: 14
64: 12
66: 14
70: 14
72: 14
74: 14
76: 14
80: 18
88: 20
90: 14
98: 17
";
