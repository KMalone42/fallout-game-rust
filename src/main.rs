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
    let root = frame.area(); // whole terminal

    // 1) Vertical split: header (fixed height) + body (rest)
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header height
            Constraint::Min(0),     // body gets the rest
        ])
        .split(root);

    let header_area = vertical[0]; // full-width header
    let body_area   = vertical[1]; // full-width body

    // 2) Inside body: main + sidebar, dynamic by percentage
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // main takes 70% of width
            Constraint::Percentage(30), // sidebar takes 30%
        ])
        .split(body_area);

    let main_area = body_chunks[0];
    let side_area = body_chunks[1];

    // 3) (Optional) Render blocks so you can see the regions
    frame.render_widget(
        Block::default().title("HEADER").borders(Borders::ALL),
        header_area,
    );
    frame.render_widget(
        Block::default().title("MAIN").borders(Borders::ALL),
        main_area,
    );
    frame.render_widget(
        Block::default().title("SIDE").borders(Borders::ALL),
        side_area,
    );
}

