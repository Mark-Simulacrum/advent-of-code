use advent_of_code::HashIter;

pub fn part1(s: &str) -> String {
    let mut out = String::new();
    for bytes in HashIter::new(s) {
        if bytes[0] == 0 && bytes[1] == 0 && hi_nib(bytes[2]) == 0 {
            out.push_str(&format!("{:x}", lo_nib(bytes[2])));
        }
        if out.len() == 8 {
            return out;
        }
    }

    unreachable!()
}

#[inline(always)]
fn hi_nib(b: u8) -> u8 {
    (b >> 4) & 0x0f
}

#[inline(always)]
fn lo_nib(b: u8) -> u8 {
    b & 0x0f
}

#[inline(always)]
fn to_pos(b: u8) -> Option<usize> {
    match b {
        0x0 => Some(0),
        0x1 => Some(1),
        0x2 => Some(2),
        0x3 => Some(3),
        0x4 => Some(4),
        0x5 => Some(5),
        0x6 => Some(6),
        0x7 => Some(7),
        0x8 => Some(8),
        0x9 => Some(9),
        _ => None,
    }
}

pub fn part2(s: &str) -> String {
    let mut out = [None, None, None, None, None, None, None, None];
    for bytes in HashIter::new(s) {
        if bytes[0] == 0 && bytes[1] == 0 && hi_nib(bytes[2]) == 0 {
            if let Some(pos) = to_pos(lo_nib(bytes[2])) {
                if pos <= 7 && out[pos].is_none() {
                    out[pos] = Some(format!("{:x}", hi_nib(bytes[3])));
                }
            }
        }
        if out.iter().all(|x| x.is_some()) {
            return out.into_iter()
                .map(|x| x.clone().unwrap())
                .fold(String::new(), |acc, x| acc + &x);
        }
    }

    unreachable!()
}

pub const INPUT: &'static str = "abbhdwsy";
