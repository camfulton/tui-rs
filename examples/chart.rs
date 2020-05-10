#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    SinSignal,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};

const DATA: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

struct App {
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new() -> App {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        App {
            signal1,
            data1,
            signal2,
            data2,
            window: [0.0, 20.0],
        }
    }

    fn update(&mut self) {
        for _ in 0..5 {
            self.data1.remove(0);
        }
        self.data1.extend(self.signal1.by_ref().take(5));
        for _ in 0..10 {
            self.data2.remove(0);
        }
        self.data2.extend(self.signal2.by_ref().take(10));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(size);
            let x_labels = vec![
                format!("{}", app.window[0]),
                format!("{}", (app.window[0] + app.window[1]) / 2.0),
                format!("{}", app.window[1]),
            ];
            let datasets = vec![
                Dataset::default()
                    .name("data2")
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&app.data1),
                Dataset::default()
                    .name("data3")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&app.data2),
            ];

            fn create_labels<'a, S>(labels: Vec<S>) -> Vec<Span<'a>>
            where
                S: 'a + AsRef<str>,
            {
                labels
                    .iter()
                    .map(|l| {
                        Span::styled(String::from(l.as_ref()), Style::default().modifier(Modifier::BOLD))
                    })
                    .collect()
            }

            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 1",
                            Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(create_labels(x_labels))
                        .bounds(app.window),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(create_labels(vec!["-20", "0", "20"]))
                        .bounds([-20.0, 20.0]),
                );
            f.render_widget(chart, chunks[0]);

            let datasets = vec![Dataset::default()
                .name("data")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&DATA)];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 2",
                            Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 5.0])
                        .labels(create_labels(vec!["0", "2.5", "5.0"])),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 5.0])
                        .labels(create_labels(vec!["0", "2.5", "5.0"])),
                );
            f.render_widget(chart, chunks[1]);

            let datasets = vec![Dataset::default()
                .name("data")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&DATA2)];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 3",
                            Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 50.0])
                        .labels(create_labels(vec!["0", "25", "50"])),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 5.0])
                        .labels(create_labels(vec!["0", "2.5", "5"])),
                );
            f.render_widget(chart, chunks[2]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
