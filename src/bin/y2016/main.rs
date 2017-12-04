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
            let bench: String = std::env::var("BENCH").ok().unwrap_or_else(|| String::from("0"));
            let should_bench = bench != "0";
            if !should_bench {
            $(
                println!("{}::part1 = {}", stringify!($day), $day::part1($day::INPUT));
                println!("{}::part2 = {}", stringify!($day), $day::part2($day::INPUT));
            )+
            } else {
            $(
                if concat!(stringify!($day), "::part1") == bench {
                    println!("benching {}", bench);
                    for _ in 0..10_000 {
                        $day::part1($day::INPUT);
                    }
                } else if concat!(stringify!($day), "::part2") == bench {
                    for _ in 0..10_000 {
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

gen!(day09, day10);
