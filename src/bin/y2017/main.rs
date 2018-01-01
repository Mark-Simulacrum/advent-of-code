#![cfg_attr(test, feature(test))]
#![feature(slice_rotate, i128, i128_type, conservative_impl_trait, drain_filter)]
#[cfg(test)]
extern crate test;
#[macro_use]
extern crate quickcheck;
#[macro_use] extern crate advent_of_code;
extern crate smallvec;
extern crate petgraph;
extern crate memchr;
extern crate itertools;
extern crate primal;

gen!(day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
     day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
     day21, day22, day23, day24, day25);
