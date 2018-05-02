use std::fmt;

pub fn modulo(a: i128, b: i128) -> u64 {
    let mut ret = a % b;
    if ret < 0 {
        ret += b;
    }
    ret as u64
}

// Algorithm taken from
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode.
#[allow(unused)]
#[derive(Copy, Clone)]
struct ExtendedGcd {
    gcd: i128,
    m: i128,
    n: i128,
}

impl ExtendedGcd {
    fn new(a: i128, b: i128) -> ExtendedGcd {
        let mut s = 0;
        let mut old_s = 1;
        let mut t = 1;
        let mut old_t = 0;
        let mut r = b;
        let mut old_r = a;
        while r != 0 {
            let q = old_r / r;
            let r1 = r;
            let s1 = s;
            let t1 = t;
            r = old_r - q * r;
            s = old_s - q * s;
            t = old_t - q * t;
            old_r = r1;
            old_s = s1;
            old_t = t1;
        }
        ExtendedGcd {
            gcd: old_r,
            m: old_s,
            n: old_t,
        }
    }
}

#[allow(unused)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut d = 0;
    while a % 2 == 0 && b % 2 == 0 {
        a /= 2;
        b /= 2;
        d += 1;
    }
    while a != b {
        if a % 2 == 0 {
            a /= 2;
        } else if b % 2 == 0 {
            b /= 2;
        } else if a > b {
            a = (a - b) / 2;
        } else {
            b = (b - a) / 2;
        }
    }
    a * 2u64.pow(d)
}

// a mod n
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Congruence {
    pub a: i128,
    pub n: i128,
}

impl fmt::Debug for Congruence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mod {}", self.a, self.n)
    }
}

impl Congruence {
    // See https://math.stackexchange.com/questions/911902/
    pub fn combine(self, other: Congruence) -> Option<Congruence> {
        let ExtendedGcd { gcd, m, n } = ExtendedGcd::new(self.n, other.n);
        if modulo(self.a, gcd) == modulo(other.a, gcd) {
            let lcm = (self.n * other.n) / gcd;
            let ret = Congruence {
                n: lcm,
                a: (other.a * m * self.n + self.a * n * other.n) / gcd,
            }.normalize();
            Some(ret)
        } else {
            None
        }
    }

    pub fn normalize(self) -> Congruence {
        let mut a = self.a;
        a -= (a / self.n) * self.n;
        if a < 0 {
            a += (a.abs() / self.n + 1) * self.n;
        }
        Congruence { n: self.n, a }
    }
}
