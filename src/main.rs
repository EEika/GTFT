use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::{self, Paragraph},
    DefaultTerminal,
};
use std::io;

use std::alloc::System;
use std::env;
use std::os::unix::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

struct FocusPeriod {
    interval: Duration,
    purpose: FocusPurpose,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal);

    let focus_periode = FocusPeriod {
        interval: Duration::new(10, 0),
        purpose: FocusPurpose::Work,
    };
    let time_now = SystemTime::now();

    let check_time = Duration::new(1, 0);

    //while focus_periode.interval.as_secs() > time_now.elapsed().unwrap().as_secs() {
    //    std::thread::sleep(check_time);
    //}
    println!("Done!");
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_dark_gray();
            frame.render_widget(greeting, frame.area());
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
enum FocusPurpose {
    Work,
    Study,
    Mindfullness,
}
