#![allow(warnings)]
#![feature(portable_simd)]
#![feature(test)]

extern crate test;

use std::time::Instant;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::timed::print_timespan;

mod day1;
mod day10;
mod day10_2;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod timed;
mod util;

// TODO: Slow days
// 2 p1 & p2
// 4 p2
// 8 p1 & p2
// 9 p2
// 10 p1 & p2
fn main() {
    let start = Instant::now();
    println!("Day 1");
    day1();
    println!("Day 2");
    day2();
    println!("Day 3");
    day3();
    println!("Day 4");
    day4();
    println!("Day 5");
    day5();
    println!("Day 6");
    day6();
    println!("Day 7");
    day7();
    println!("Day 8");
    day8();
    println!("Day 9");
    day9();
    println!("Day 10");
    day10();
    println!("Day 11");
    day11();
    println!("Day 12");
    day12();
    print_timespan("Total", Instant::now() - start);
}
