use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use ratatui::{prelude::*, widgets::*};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let root = frame.area();

    // 1) First: horizontally center a 100-column wide content area
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),       // left padding
            Constraint::Length(100),  // our 100-column content area
            Constraint::Min(0),       // right padding
        ])
        .split(root);
    let content = horizontal[1]; // 100-column wide area in the middle

    // 2) Split that content vertically: header + body (24 rows)
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // header height (frame 1)
            Constraint::Length(24),  // body area for frames 2 + 3
            // If the terminal is taller, extra rows are left unused
        ])
        .split(content);

    let header_area = vertical[0]; // frame 1
    let body_area   = vertical[1]; // area that will be split into frame 2 + 3

    // 3) Split body horizontally into main (80) and side (20)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(80),  // frame 2: main
            Constraint::Length(20),  // frame 3: sidebar
        ])
        .split(body_area);

    let main_area = body_chunks[0]; // frame 2
    let side_area = body_chunks[1]; // frame 3

    // 4) Render something into each "frame"
    let header = Block::default()
        .title("Header (frame 1)")
        .borders(Borders::ALL);

    let main = Block::default()
        .title("Main (frame 2: 80x24)")
        .borders(Borders::ALL);

    let side = Block::default()
        .title("Sidebar (frame 3: 20x24)")
        .borders(Borders::ALL);

    frame.render_widget(header, header_area);
    frame.render_widget(main,   main_area);
    frame.render_widget(side,   side_area);
}
