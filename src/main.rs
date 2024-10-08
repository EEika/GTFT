#![allow(unused_imports, unused_variables)]
use notify_rust::{Notification, Timeout};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::{Color, Style, Stylize},
    widgets::LineGauge,
    DefaultTerminal,
};
use std::env;
use std::io;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct FocusPeriode {
    interval: Duration,
    purpose: FocusPurpose,
    start_time: Instant,
}

impl FocusPeriode {
    fn time_remaining(&self) -> u64 {
        self.interval.as_secs() - self.start_time.elapsed().as_secs()
    }

    fn ratio_remaining(&self) -> f64 {
        self.start_time.elapsed().as_secs() as f64 / self.interval.as_secs() as f64
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut focus_time = 5;
    let mut focus_purpose = FocusPurpose::Work;

    if let Some(ft) = args.get(1) {
        match ft.parse() {
            Ok(x) => focus_time = x,
            Err(e) => focus_time = 5,
        }
    }
    if let Some(p) = args.get(2) {
        match p.as_str() {
            "Work" => focus_purpose = FocusPurpose::Work,
            "Study" => focus_purpose = FocusPurpose::Study,
            "Mindfullness" => focus_purpose = FocusPurpose::Mindfullness,
            _ => focus_purpose = FocusPurpose::Work,
        }
    }

    let focus_periode = FocusPeriode {
        interval: Duration::new(focus_time, 0),
        purpose: focus_purpose,
        start_time: Instant::now(),
    };

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal, &focus_periode);
    ratatui::restore();

    Notification::new()
        .summary("GTFT")
        .body(&format!(
            "{} session finished. Well done!",
            focus_periode.purpose.display()
        ))
        //.timeout(Timeout::Milliseconds(6000)) //milliseconds
        .show()
        .unwrap();
    println!("Done!\x07");
    app_result
}

fn run(mut terminal: DefaultTerminal, periode: &FocusPeriode) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let progress = LineGauge::default()
                .filled_style(Style::default().fg(Color::Blue))
                .label(format!(
                    "Time remaining: {}:{}",
                    periode.time_remaining() / 60,
                    periode.time_remaining() % 60
                ))
                .ratio(periode.ratio_remaining());
            //.white()
            //.on_dark_gray();
            frame.render_widget(progress, frame.area());
        })?;
        if periode.time_remaining() == 0 {
            return Ok(());
        }
        //if let event::Event::Key(key) = event::read()? {
        //    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
        //        return Ok(());
        //    }
        //}
    }
}
#[derive(Debug)]
enum FocusPurpose {
    Work,
    Study,
    Mindfullness,
}

impl FocusPurpose {
    fn display(&self) -> &str {
        match self {
            FocusPurpose::Work => "Work",
            FocusPurpose::Study => "Study",
            FocusPurpose::Mindfullness => "Mindfullness",
        }
    }
}
