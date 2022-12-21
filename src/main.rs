use std::env;

mod _01;
mod _02;
mod _03;
mod _04;
mod _05;
mod _06;
mod _07;
mod _08;
mod _09;
mod _10;
mod _11;
mod _12;

mod _test;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day.as_str() {
        "1" => _01::run(),
        "2" => _02::run(),
        "3" => _03::run(),
        "4" => _04::run(),
        "5" => _05::run(),
        "6" => _06::run(),
        "7" => _07::run(),
        "8" => _08::run(),
        "9" => _09::run(),
        "10" => _10::run(),
        "11" => _11::run(),
        "12" => _12::run(),

        "test" => _test::run(),
        _ => println!("Invalid day"),
    }
}
