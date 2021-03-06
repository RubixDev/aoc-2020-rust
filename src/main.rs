use std::time;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let start = time::Instant::now();

    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();

    println!("--------------\nExecution took {:?}", start.elapsed());
}
