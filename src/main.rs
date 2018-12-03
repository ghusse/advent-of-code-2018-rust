extern crate elapsed;
extern crate regex;

mod day_1;
mod day_2;
mod day_3;
pub mod measure;

fn main() {
    day_1::solve();
    println!();

    day_2::solve();
    println!();

    day_3::solve();
    println!();
}
