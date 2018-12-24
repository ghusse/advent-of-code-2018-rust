extern crate core;
extern crate datetime;
extern crate elapsed;
extern crate quickersort;
extern crate regex;

use std::env;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
pub mod measure;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 || args[1].eq("1") {
        day_1::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("2") {
        day_2::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("3") {
        day_3::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("4") {
        day_4::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("5") {
        day_5::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("6") {
        day_6::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("7") {
        day_7::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("8") {
        day_8::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("9") {
        day_9::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("10") {
        day_10::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("11") {
        day_11::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("12") {
        day_12::solve();
        println!();
    }

    if args.len() == 1 || args[1].eq("13") {
        day_13::solve();
        println!();
    }
}
