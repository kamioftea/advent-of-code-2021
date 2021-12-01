mod day_1;
use std::time::Instant;
use std::io::{self, Write};

extern crate core;

#[macro_use]
extern crate text_io;
extern crate regex;
extern crate proc_macro;
extern crate im;
extern crate either;

fn main() {
    print!("Which day? (0 to run all): ");
    io::stdout().flush().unwrap();

    let day: i32 = read!();
    let days:Vec<Box<dyn Fn()->()>> = vec!(
        Box::new(|| day_1::run()),
    );

    let start = Instant::now();
    match days.get((day - 1) as usize) {
        Some(solution) => solution(),
        None if day == 0 => days.iter().enumerate().for_each(|(i, solution)| {
            let start = Instant::now();
            println!("==== Day {} ====", i + 1);
            solution();
            println!("-- took {:.2?}", start.elapsed());
        }),
        None => println!("Invalid Day {}", day)
    }

    println!();
    println!("Finished in {:.2?}", start.elapsed());
}

