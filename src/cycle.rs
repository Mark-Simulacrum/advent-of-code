
#[derive(Debug, Copy, Clone)]
pub struct Cycle {
    /// Smallest index such that this value will occur infinitely often.
    mu: usize,
    /// The length of the cycle. That is, how many values are in each "loop".
    lambda: usize,
}

impl Cycle {
    pub fn start(self) -> usize {
        self.mu
    }

    pub fn length(self) -> usize {
        self.lambda
    }
}

pub fn length<I: Clone + PartialEq + Eq, F: FnMut(I) -> I>(x0: I, mut f: F) -> usize {
    // main phase: search successive powers of two
    let mut lam = 1;
    let mut power = 1;
    let mut tortoise = x0.clone();
    let mut hare = f(x0);
    while tortoise != hare {
        if power == lam { // time to start new power
            tortoise = hare.clone();
            power *= 2;
            lam = 0;
        }
        hare = f(hare);
        lam += 1;
    }
    lam
}

pub fn find<I: Clone + PartialEq + Eq, F: FnMut(I) -> I>(x0: I, mut f: F) -> Cycle {
    let lam = length(x0.clone(), |x| f(x));
    // Find the position of the first repetition of length lam
    let mut hare = x0.clone();
    let mut tortoise = x0;
    for _ in 0..lam {
        hare = f(hare);
    }

    // distance between hare and tortoise is lam.
    let mut mu = 0;
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1;
    }
    Cycle { mu, lambda: lam }
}
