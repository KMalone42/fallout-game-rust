use ratatui::prelude::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph,
    Table, Row, Cell,
};
use ratatui::Frame;
use ratatui::style::{Color, Style};

use crate::state::{App, Focus};

pub fn render(frame: &mut Frame, app: &App, side_area_out: &mut Rect) {
    let root = frame.area();
    let mut ui = root;

    // 4:3-ish aspect ratio logic
    if ui.height * 10 / 3 <= ui.width {
        ui.width = ui.height * 10 / 3; // width shrinks to fit height
    } else {
        ui.height = ui.width * 3 / 10; // height shrinks to fit width
    }

    // Vertical: header + body
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(0),     // body
        ])
        .split(ui);

    let header_area = vertical[0];
    let body_area   = vertical[1];

    // Horizontal: main + sidebar
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(body_area);

    let main_area = body_chunks[0];
    let side_area = body_chunks[1];

    // expose sidebar rect to app loop for hit-testing
    *side_area_out = side_area;

    draw_header(frame, header_area, &app.header_text);
    draw_main(frame, main_area, app);
    draw_side(frame, side_area, app);
}

// ----------------------------------------------------------------------------


fn draw_header(frame: &mut Frame, area: Rect, header_text: &str) {
    let block = Block::default().title("HEADER").borders(Borders::ALL);

    let content = Paragraph::new(header_text.to_string())
        .block(block);

    frame.render_widget(content, area);
}


fn draw_main(frame: &mut Frame, area: Rect, app: &App) {
    let block = match app.focus {
        Focus::Main => {
            Block::default().title("Main [active]").borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
        }
        _ => Block::default().title("Main").borders(Borders::ALL),
    };

    // ---------------------------------------------------------------------
    // Header row
    // ---------------------------------------------------------------------
    let header = Row::new([
        Cell::from("Col 0"),   // narrow
        Cell::from("Col 1"),   // wide
        Cell::from("Col 2"),   // narrow
        Cell::from("Col 3"),   // wide
    ]).style(Style::default().add_modifier(Modifier::BOLD));

    // ---------------------------------------------------------------------
    // 16 rows of sample data (you replace this however you want)
    // ---------------------------------------------------------------------
    let rows = (0..16).map(|i| {
        Row::new(vec![
            Cell::from(format!("r{i}c0")),
            Cell::from(format!("row {i} wide text col1")),
            Cell::from(format!("c2-{i}")),
            Cell::from(format!("wide col3 text for row {i}")),
        ])
    });

    // ---------------------------------------------------------------------
    // Build table
    // ---------------------------------------------------------------------
    let widths = [
        Constraint::Length(6),      // col 0 (narrow)
        Constraint::Percentage(40), // col 1 (WIDE)
        Constraint::Length(8),      // col 2 (narrow)
        Constraint::Percentage(40), // col 3 (WIDE)
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .column_spacing(1);

    frame.render_widget(table, area);
}


fn draw_side(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .side_items
        .iter()
        .map(|s| ListItem::new(s.as_str()))
        .collect();

    let block = match app.focus {
        Focus::Side => {
            Block::default().title("Side [active]").borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
        }
        _ => Block::default().title("Side").borders(Borders::ALL),
    };

    let list = List::new(items)
        .block(block)
        .highlight_symbol("➤ ")
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD),
        );

    let mut state = ListState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(list, area, &mut state);
}
