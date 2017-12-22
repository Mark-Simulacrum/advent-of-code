#![cfg_attr(test, feature(test))]
#![feature(i128, i128_type, conservative_impl_trait, drain_filter)]
#[cfg(test)]
extern crate test;
#[macro_use]
extern crate quickcheck;
#[macro_use] extern crate advent_of_code;
extern crate smallvec;
extern crate petgraph;
extern crate memchr;

gen!(day01, day02, day03, day04, day05, day06, day07, day08, day09,
     day10, day11, day12, day13, day14);
