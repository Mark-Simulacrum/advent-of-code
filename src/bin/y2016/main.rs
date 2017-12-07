#![cfg_attr(test, feature(test))]
#![feature(option_filter, drain_filter)]
#[cfg(test)]
extern crate test;
#[macro_use] extern crate advent_of_code;
extern crate smallvec;
#[macro_use] extern crate bitflags;
extern crate itertools;

gen!(day09, day10, day11);
