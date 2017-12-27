#[macro_use] extern crate failure;
extern crate memchr;

use memchr::memchr;
use memchr::memchr2;

#[derive(Fail, Debug, PartialEq, Eq)]
pub enum ParserError {
    #[fail(display = "Unexpected end of input")]
    Eof,
    #[fail(display = "Expected char {}, got {:?}", _0, _1)]
    Unexpected(String, Option<char>),
}


#[derive(Copy, Clone)]
pub struct Parser<'a> {
    idx: usize,
    input: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new<'b>(input: &'b [u8]) -> Parser<'b> {
        Parser {
            idx: 0,
            input: input,
        }
    }

    #[inline(always)]
    pub fn at_end(&self) -> bool {
        self.idx == self.input.len()
    }

    #[inline(always)]
    pub fn cur(&self) -> Option<u8> {
        self.input.get(self.idx).map(|x| *x)
    }

    #[inline(always)]
    pub fn read(&mut self) -> Option<u8> {
        let out = self.cur();
        if out.is_some() {
            self.idx += 1;
        }
        out
    }

    #[inline(always)]
    pub fn advance(&mut self) {
        if self.at_end() {
            panic!("advanced past input length");
        }
        self.idx += 1;
    }

    /// If needle isn't present, consumes until EOF.
    pub fn consume_until(&mut self, c: u8) -> usize {
        if let Some(idx) = memchr(c, &self.input[self.idx..]) {
            self.idx += idx;
            idx
        } else {
            let ret = self.input.len() - self.idx;
            self.idx = self.input.len();
            ret
        }
    }

    pub fn consume_until_or_stop(&mut self, c: u8) -> Option<usize> {
        if let Some(idx) = memchr(c, &self.input[self.idx..]) {
            self.idx += idx;
            Some(idx)
        } else {
            None
        }
    }

    pub fn consume_until2_or_stop(&mut self, a: u8, b: u8) -> Option<usize> {
        if let Some(idx) = memchr2(a, b, &self.input[self.idx..]) {
            self.idx += idx;
            Some(idx)
        } else {
            None
        }
    }

    pub fn consume_bytes_until(&mut self, c: u8) -> &'a [u8] {
        let orig_idx = self.idx;
        let _ = self.consume_until(c);
        &self.input[orig_idx..self.idx]
    }

    pub fn expect(&mut self, needle: &[u8]) -> Result<bool, ParserError> {
        match self.consume_bytes(needle.len()).map(|b| b == needle) {
            Ok(eq) => {
                if !eq {
                    self.idx -= needle.len();
                }
                Ok(eq)
            },
            Err(e) => Err(e),
        }
    }

    pub fn consume_bytes(&mut self, n: usize) -> Result<&[u8], ParserError> {
        if self.idx + n > self.input.len() {
            return Err(ParserError::Eof);
        }

        let out = &self.input[self.idx..self.idx + n];
        self.idx += n;
        Ok(out)
    }

    pub fn consume(&mut self, x: u8) -> Result<(), ParserError> {
        let next = self.cur();
        if next == Some(x) {
            self.idx += 1;
            Ok(())
        } else {
            return Err(ParserError::Unexpected(format!("char {}", x as char), next.map(|c| c as char)));
        }
    }

    pub fn consume_signed_number(&mut self) -> Result<i64, ParserError> {
        let sign = if self.cur() == Some(b'-') {
            self.idx += 1;
            -1
        } else { 1 };
        Ok(self.consume_number()? as i64 * sign)
    }

    pub fn consume_number(&mut self) -> Result<u64, ParserError> {
        let mut i = 0;
        let mut out = 0;
        while let Some(next) = self.cur().and_then(|c| (c as char).to_digit(10)) {
            self.idx += 1;
            out *= 10;
            out += next as u64;
            i += 1;
        }
        if i == 0 {
            return Err(ParserError::Unexpected(format!("number"), self.cur().map(|c| c as char)));
        }
        Ok(out)
    }
}

#[test]
fn parser_number() {
    assert_eq!(Parser::new(b"100").consume_number(), Ok(100));
}

#[test]
fn parser_consume_until() {
    assert_eq!(Parser::new(b"aaax").consume_until(b'x'), 3);
}

#[test]
fn parser_consume_until_no_exist() {
    assert_eq!(Parser::new(b"aaa").consume_until(b'x'), 3);
}

#[macro_export]
macro_rules! gen {
    ($($day:ident),+) => {
        $(
            mod $day;
        )+

        fn main() {
            let bench: String = std::env::var("BENCH").ok().unwrap_or_else(|| String::from("0"));
            let filter: String = std::env::args().nth(1).unwrap_or_default();
            let iters: usize = std::env::var("BENCH_TIMES").ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(1000);
            let should_bench = bench != "0";
            if !should_bench {
            $(
                if stringify!($day).contains(&filter) {
                    {
                        let start = ::std::time::Instant::now();
                        let res = $day::part1($day::INPUT);
                        let elapsed = start.elapsed();
                        println!("{}::part1 = {} ({}s {}ns)",
                            stringify!($day), res, elapsed.as_secs(), elapsed.subsec_nanos());
                    }
                    {
                        let start = ::std::time::Instant::now();
                        let res = $day::part2($day::INPUT);
                        let elapsed = start.elapsed();
                        println!("{}::part2 = {} ({}s {}ns)",
                            stringify!($day), res, elapsed.as_secs(), elapsed.subsec_nanos());
                    }
                }
            )+
            } else {
            $(
                if concat!(stringify!($day), "::part1") == bench {
                    println!("benching {}", bench);
                    for _ in 0..iters {
                        $day::part1($day::INPUT);
                    }
                } else if concat!(stringify!($day), "::part2") == bench {
                    for _ in 0..iters {
                        $day::part2($day::INPUT);
                    }
                }
            )+
            }
        }

        #[cfg(test)]
        mod part1 {
            use test::Bencher;
            $(
            #[bench]
            fn $day(b: &mut Bencher) {
                b.iter(|| ::$day::part1(::$day::INPUT));
            }
            )+
        }

        #[cfg(test)]
        mod part2 {
            use test::Bencher;
            $(
            #[bench]
            fn $day(b: &mut Bencher) {
                b.iter(|| ::$day::part2(::$day::INPUT));
            }
            )+
        }
    }
}
