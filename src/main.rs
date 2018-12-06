extern crate core;
extern crate datetime;
extern crate elapsed;
extern crate quickersort;
extern crate regex;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
pub mod measure;

fn main() {
    day_1::solve();
    println!();

    day_2::solve();
    println!();

    day_3::solve();
    println!();

    day_4::solve();
    println!();

    day_5::solve();
    println!();
}
