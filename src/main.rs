use std::alloc::System;
use std::env;
use std::os::unix::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

struct FocusPeriod {
    interval: Duration,
    purpose: FocusPurpose,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let focus_periode = FocusPeriod {
        interval: Duration::new(10, 0),
        purpose: FocusPurpose::Work,
    };
    let time_now = SystemTime::now();

    let check_time = Duration::new(1, 0);

    println!(
        "Starting focus for: {}:{}",
        focus_periode.interval.as_secs() / 60,
        focus_periode.interval.as_secs() % 60
    );
    while focus_periode.interval.as_secs() > time_now.elapsed().unwrap().as_secs() {
        std::thread::sleep(check_time);
    }
    println!("Done! Well done!");
}

enum FocusPurpose {
    Work,
    Study,
    Mindfullness,
}
