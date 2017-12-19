#![cfg_attr(test, feature(test))]
#![feature(plugin, conservative_impl_trait, drain_filter)]
#![plugin(quickcheck_macros)]
#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate quickcheck;
#[macro_use] extern crate advent_of_code;
extern crate smallvec;
extern crate petgraph;
extern crate memchr;

gen!(day01, day02, day03, day04, day05, day06, day07, day08, day09,
     day10, day11);
