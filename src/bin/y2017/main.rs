#![feature(const_fn, test, slice_rotate, i128, drain_filter, stdsimd)]

#[macro_use]
extern crate advent_of_code;
extern crate fnv;
extern crate itertools;
extern crate memchr;
extern crate petgraph;
extern crate primal;
#[macro_use]
extern crate quickcheck;
extern crate smallvec;
extern crate test;

gen!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
);
