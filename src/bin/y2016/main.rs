#![cfg_attr(test, feature(test))]
#![feature(option_filter)]
#[cfg(test)]
extern crate test;
#[allow(unused_imports)]
#[macro_use] extern crate failure;
extern crate advent_of_code;
extern crate smallvec;

macro_rules! gen {
    ($($day:ident),+) => {
        $(
            mod $day;
        )+

        fn main() {
            let bench = std::env::var("BENCH").ok().map_or(false, |c| c == "1");
            if !bench {
            $(
                println!("{}::part1 = {}", stringify!($day), $day::part1($day::INPUT));
                println!("{}::part2 = {}", stringify!($day), $day::part2($day::INPUT));
            )+
            } else {
                for _ in 0..1000 {
            $(
                    $day::part1($day::INPUT);
                    $day::part2($day::INPUT);
            )+
                }
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

gen!(day09, day10);
