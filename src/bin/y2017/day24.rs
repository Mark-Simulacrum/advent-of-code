fn valid_connectors<'a>(ports: &'a [(u8, u8)], to: u8) -> impl Iterator<Item=(usize, u8)> + 'a {
    // return the position and the unused connector
    ports.iter().enumerate().filter_map(move |(i, port)| {
        if port.0 == to {
            Some((i, port.1))
        } else if port.1 == to {
            Some((i, port.0))
        } else {
            None
        }
    })
}

fn eval(s: &str) -> (Vec<Vec<(usize, u8)>>, Vec<(u8, u8)>) {
    let ports = s.trim().lines().map(|line| {
        let mut i = line.split('/').map(|x| x.parse::<u8>().unwrap());
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        if a > b { (a, b) } else { (b, a) }
    }).collect::<Vec<_>>();
    let mut handled = Vec::new();
    let mut bridges = ports.iter().enumerate()
        .filter(|&(_, x)| x.1 == 0)
        .map(|(i, c)| vec![(i, c.0)]).collect::<Vec<_>>();
    while let Some(bridge) = bridges.pop() {
        for (pos, conn) in valid_connectors(&ports, bridge.last().unwrap().1) {
            // if this bridge already used this connector
            if bridge.iter().any(|&(p, _)| p == pos) {
                continue;
            }
            let mut next = bridge.clone();
            next.push((pos, conn));
            bridges.push(next);
        }
        handled.push(bridge);
    }
    (handled, ports)
}

pub fn part1(s: &str) -> usize {
    let (handled, ports) = eval(s);
    handled.into_iter().map(|bridge| {
        let mut strength = 0;
        for (pos, _) in bridge {
            strength += ports[pos].0 as usize + ports[pos].1 as usize;
        }
        strength
    }).max().unwrap()
}

pub fn part2(s: &str) -> usize {
    let (handled, ports) = eval(s);
    handled.into_iter().map(|bridge| {
        let mut strength = 0;
        for &(pos, _) in &bridge {
            strength += ports[pos].0 as usize + ports[pos].1 as usize;
        }
        (bridge.len(), strength)
    }).max().unwrap().1
}

#[test]
fn part1_1() {
    assert_eq!(part1(EXAMPLE), 31);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 1906);
}

#[test]
fn part2_1() {
    assert_eq!(part2(EXAMPLE), 19);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 1824);
}

#[cfg(test)]
static EXAMPLE: &str = "
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";

pub static INPUT: &str = "
31/13
34/4
49/49
23/37
47/45
32/4
12/35
37/30
41/48
0/47
32/30
12/5
37/31
7/41
10/28
35/4
28/35
20/29
32/20
31/43
48/14
10/11
27/6
9/24
8/28
45/48
8/1
16/19
45/45
0/4
29/33
2/5
33/9
11/7
32/10
44/1
40/32
2/45
16/16
1/18
38/36
34/24
39/44
32/37
26/46
25/33
9/10
0/29
38/8
33/33
49/19
18/20
49/39
18/39
26/13
19/32
";
