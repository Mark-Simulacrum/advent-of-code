#[derive(Copy, Clone, Debug)]
struct Generator {
    value: u64,
    factor: u64,
}

impl Generator {
    fn next(&mut self) -> u64 {
        self.value *= self.factor;
        self.value %= 2_147_483_647;

        self.value
    }

    fn next_multiple_4(&mut self) -> u64 {
        self.next();
        while self.value % 4 != 0 {
            self.next();
        }
        self.value
    }

    fn next_multiple_8(&mut self) -> u64 {
        self.next();
        while self.value % 8 != 0 {
            self.next();
        }
        self.value
    }
}

fn parse(s: &str) -> (Generator, Generator) {
    let mut values = s.trim().lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<u64>().unwrap());
    let a = Generator { value: values.next().unwrap(), factor: 16807 };
    let b = Generator { value: values.next().unwrap(), factor: 48271 };
    (a, b)
}

pub fn part1(s: &str) -> u16 {
    let (mut a, mut b) = parse(s);
    let mask = 0b1111_1111_1111_1111;
    let mut count = 0;
    for _ in 0..40_000_000 {
        let v_a = a.next();
        let v_b = b.next();
        if (v_a & mask) == (v_b & mask) {
            count += 1;
        }
    }
    count
}

pub fn part2(s: &str) -> usize {
    let (mut a, mut b) = parse(s);
    let mask = 0b1111_1111_1111_1111;
    let mut count = 0;
    for _ in 0..5_000_000 {
        let v_a = a.next_multiple_4();
        let v_b = b.next_multiple_8();
        if (v_a & mask) == (v_b & mask) {
            count += 1;
        }
    }
    count
}

#[test]
fn part1_1() {
    assert_eq!(part1("65\n8921"), 588);
}

#[test]
fn part2_1() {
    assert_eq!(part2("65\n8921"), 309);
}

#[test]
fn part1_actual() {
    assert_eq!(part1(INPUT), 569);
}

#[test]
fn part2_actual() {
    assert_eq!(part2(INPUT), 298);
}

pub static INPUT: &str = "
Generator A starts with 116
Generator B starts with 299
";
