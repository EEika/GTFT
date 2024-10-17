#![allow(unused_imports, unused_variables)]
use notify_rust::{Notification, Timeout};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers},
    layout::Flex,
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{block::Title, Block, Borders, Gauge, LineGauge, Paragraph},
    DefaultTerminal,
};
use std::env;
use std::io;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum FocusPurpose {
    Work,
    Study,
    Mindfullness,
}

#[derive(Debug)]
enum FocusStatus {
    InProgress,
    Finished,
    Overtime,
    Canceled,
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

#[derive(Debug)]
struct FocusPeriode {
    interval: Duration,
    purpose: FocusPurpose,
    start_time: Instant,
    status : FocusStatus
}

impl FocusPeriode {
    fn time_remaining(&self) -> u64 {
        self.interval.as_secs() - self.start_time.elapsed().as_secs()
    }

    fn ratio_remaining(&self) -> f64 {
        self.start_time.elapsed().as_secs() as f64 / self.interval.as_secs() as f64
    }

    fn time_over(&self) -> u64 {
        self.start_time.elapsed().as_secs() - self.interval.as_secs()
    }
    fn periode_finished(&self) -> bool {

        self.start_time.elapsed().as_secs() >= self.interval.as_secs()
    }
    fn update_status(&mut self, new_status : FocusStatus){
        self.status = new_status;
    }

}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut focus_time_min: u64 = 25;
    let mut focus_time_sec: u64 = 0;
    let mut focus_purpose = FocusPurpose::Work;

    if let Some(ft) = args.get(1) {
        match ft.parse() {
            Ok(m) => focus_time_min = m,
            Err(e) => (),
        }
    }
    if let Some(ft) = args.get(2) {
        match ft.parse() {
            Ok(s) => focus_time_sec = s,
            Err(e) => (),
        }
    }
    if let Some(p) = args.get(3) {
        match p.as_str() {
            "Work" => focus_purpose = FocusPurpose::Work,
            "0" => focus_purpose = FocusPurpose::Work,
            "Study" => focus_purpose = FocusPurpose::Study,
            "1" => focus_purpose = FocusPurpose::Study,
            "Mindfullness" => focus_purpose = FocusPurpose::Mindfullness,
            "2" => focus_purpose = FocusPurpose::Mindfullness,
            _ => (),
        }
    }

    let mut focus_periode = FocusPeriode {
        interval: Duration::new((focus_time_min * 60) + focus_time_sec, 0),
        purpose: focus_purpose,
        start_time: Instant::now(),
        status : FocusStatus::InProgress,
    };

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal, &mut focus_periode);
    ratatui::restore();

    match app_result? {
        FocusStatus::Finished => {
            println!("Done!\x07");
        }
        FocusStatus::Canceled => (),
        _ => (),
    };
    Ok(())
}

fn run(mut terminal: DefaultTerminal, periode: &mut FocusPeriode) -> io::Result<FocusStatus> {
    loop {
        terminal.draw(|frame| {
            let area = center(
                frame.area(),
                Constraint::Percentage(80),
                Constraint::Percentage(80),
            );
            let main_block_base =
                Gauge::default()
                .block(
                    Block::new()
                    .title(
                        Title::from("(Ain't Nobody) Got Time For That")
                        .alignment(Alignment::Center),
                    )
                    .borders(Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::Blue));
            let progress = match periode.status {
                FocusStatus::InProgress => {
                    if periode.periode_finished(){
                        periode.update_status(FocusStatus::Finished);
                    }
                    main_block_base.clone()
                        .label(
                            format!(
                                "Time remaining: {}:{:0>2}",
                                periode.time_remaining() / 60,
                                periode.time_remaining() % 60
                            )
                        )
                        .ratio(periode.ratio_remaining())
                },
                FocusStatus::Finished => {
                    if periode.periode_finished(){
                        periode.update_status(FocusStatus::Overtime);
                    }
                    Notification::new()
                        .summary("GTFT")
                        .body(&format!(
                                "{} session finished. Well done!",
                                periode.purpose.display()
                        ))
                        .timeout(Timeout::Never)
                        .show()
                        .unwrap();
                    main_block_base.clone()
                        .label( "DONE!")
                        .ratio(1.0) //.white()
                },
                FocusStatus::Overtime => {

                    main_block_base.clone()
                        .label(
                            format!(
                                "OVERTIME! {}:{:0>2} + {}:{:0>2}",
                                periode.interval.as_secs() / 60,
                                periode.interval.as_secs() % 60,
                                periode.time_over() / 60,
                                periode.time_over() % 60
                            )
                        )
                        .ratio(1.0)
                },
                FocusStatus::Canceled => {main_block_base.clone()},

            };
            frame.render_widget(progress, area);

        })?;
        // if periode.time_remaining() == 0 {
        //     return Ok(FocusStatus::Finished);
        // }
        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('c')
                        && key.modifiers == KeyModifiers::CONTROL
                {
                    // return Ok(FocusStatus::Finished)
                    match periode.status {
                        FocusStatus::Finished | FocusStatus::Overtime => return Ok(FocusStatus::Finished),
                        FocusStatus::InProgress | FocusStatus::Canceled => return Ok(FocusStatus::Canceled),
                    }
                }
            }
        }
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
