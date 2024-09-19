use std::alloc::System;
use std::env;
use std::time::{Duration, SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();

    let time_now = SystemTime::now();
    for i in args {
        println!("{i}");
    }
    println!("{:?}", time_now.elapsed().unwrap().as_nanos());
}
