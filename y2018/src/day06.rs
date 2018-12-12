use aoc_macro::{generator, solution};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

#[generator]
fn generator(input: &str) -> Vec<Coordinate> {
    input.trim().lines().map(|l| {
        let x = l[..l.find(",").unwrap()].parse::<isize>().unwrap();
        let y = l[l.find(" ").unwrap() + 1..].parse::<isize>().unwrap();
        Coordinate { x, y }
    }).collect()
}

impl Coordinate {
    fn distance(&self, to: Coordinate) -> usize {
        (to.x - self.x).abs() as usize + (to.y - self.y).abs() as usize
    }
}

fn closest_to(point: Coordinate, landmarks: &[Coordinate]) -> Option<(usize, Coordinate)> {
    let mut distances = Vec::with_capacity(landmarks.len());
    for (idx, landmark) in landmarks.iter().enumerate() {
        distances.push((landmark.distance(point), landmark, idx));
    }

    distances.sort_by_key(|(d, ..)| *d);

    // If we have more than one distance with the same closeness then we must reject this point
    if distances[0].0 == distances[1].0 {
        return None;
    }

    Some((distances[0].2, *distances[0].1))
}

#[solution(part1,
    example_input = generator(EXAMPLE),
    example = 17)]
fn part1(landmarks: Vec<Coordinate>) -> usize {
    let mut skip = vec![false; landmarks.len()];
    let mut far_far_away = Vec::new();
    for x in -1000..=1000 {
        far_far_away.push(Coordinate { x, y: -1000 });
        far_far_away.push(Coordinate { x, y: 1000 });
    }
    for y in -1000..=1000 {
        far_far_away.push(Coordinate { y, x: -1000 });
        far_far_away.push(Coordinate { y, x: 1000 });
    }
    for &pt in &far_far_away {
        if let Some((idx, _)) = closest_to(pt, &landmarks) {
            skip[idx] = true;
        }
    }
    let skip = skip;
    let mut spaces_closest = vec![0; landmarks.len()];
    for x in -1000..=1000 {
        for y in -1000..=1000 {
            let c = Coordinate { x, y };
            if let Some((idx, _)) = closest_to(c, &landmarks) {
                if skip[idx] { continue; }
                spaces_closest[idx] += 1;
            }
        }
    }
    *spaces_closest.iter().max().unwrap()
}

#[solution(part2,
    example_input = generator(EXAMPLE),
    example = 5554416)]
fn part2(landmarks: Vec<Coordinate>) -> usize {
    let size = 10_000;
    let mut region = 0;
    for x in -size..=size {
        for y in -size..=size {
            let c = Coordinate { x, y };
            let mut sum = 0;
            for landmark in &landmarks {
                sum += landmark.distance(c);
                if sum >= 10_000 {
                    break;
                }
            }
            if sum < 10_000 {
                region += 1;
            }
        }
    }
    region
}

static EXAMPLE: &str = "
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

static INPUT: &str = "
67, 191
215, 237
130, 233
244, 61
93, 93
145, 351
254, 146
260, 278
177, 117
89, 291
313, 108
145, 161
143, 304
329, 139
153, 357
217, 156
139, 247
304, 63
202, 344
140, 302
233, 127
260, 251
235, 46
357, 336
302, 284
313, 260
135, 40
95, 57
227, 202
277, 126
163, 99
232, 271
130, 158
72, 289
89, 66
94, 111
210, 184
139, 58
99, 272
322, 148
209, 111
170, 244
230, 348
112, 200
287, 55
320, 270
53, 219
42, 52
313, 205
166, 259
";
