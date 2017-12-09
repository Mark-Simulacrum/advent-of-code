#![cfg_attr(test, feature(test))]
#![feature(conservative_impl_trait, drain_filter)]
#[cfg(test)]
extern crate test;
#[macro_use] extern crate advent_of_code;
extern crate smallvec;
extern crate petgraph;

gen!(day01, day02, day03, day04, day05, day06, day07, day08);
