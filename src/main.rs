#![allow(warnings)]
#![feature(portable_simd)]

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

mod day1;
mod day10;
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
// 10 p1
fn main() {
    day10();
}
