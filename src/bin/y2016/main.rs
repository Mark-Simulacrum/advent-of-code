#![feature(test, drain_filter, in_band_lifetimes)]

#[macro_use]
extern crate advent_of_code;
#[macro_use]
extern crate bitflags;
extern crate itertools;
extern crate itoa;
extern crate md5;
extern crate openssl;
extern crate permutohedron;
extern crate petgraph;
extern crate smallvec;
extern crate test;

gen!(
    day01, day02, day03, day04, day05, day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24, day25,
);
