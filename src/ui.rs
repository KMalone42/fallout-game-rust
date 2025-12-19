use ratatui::prelude::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph,
    Table, Row, Cell, Clear, Wrap, TableState,
};
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style};

use crate::app::{App, Focus, DebugLog};

pub fn render(frame: &mut Frame, app: &mut App) {
    let root = frame.area();
    let mut ui = root;

    // 4:3-ish aspect ratio logic
    if ui.height * 10 / 3 <= ui.width {
        ui.width = ui.height * 10 / 3; // width shrinks to fit height
    } else {
        ui.height = ui.width * 3 / 10; // height shrinks to fit width
    }

    // Describe Layout
    // ------------------------------------------------------------------------
    // Vertical: header + body
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // header 4 inner 2 border
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

    let side_area_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(10),
        ])
        .split(side_area);

    let history_area = side_area_chunks[0];
    let input_area   = side_area_chunks[1];

    // End layout description
    //-------------------------------------------------------------------------

    // expose sidebar rect to app loop for hit-testing
    // *side_area_out = side_area;

    draw_header(frame, header_area, app);
    draw_main(frame, main_area, app);
    draw_side(frame, side_area, history_area, input_area, app);

    // Popups
    if app.game_over {
        draw_game_over(frame, root);
    }

    if app.show_help {
        let area = frame.area();
        draw_help(frame, area);
    }

    if app.show_debug {
        let area = frame.area();
        draw_debug(frame, area, &app.debug);
    }

}
// End render 'ui runtime'
// ----------------------------------------------------------------------------


fn draw_header(frame: &mut Frame, area: Rect, app: &mut App) {
    // App is being passed in for health, title and whatever
    let block = match app.focus {
        Focus::Head => Block::default()
            .title("Header [active]")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
        _ => Block::default().title("Header").borders(Borders::ALL),
    };

    let text = Text::from(vec![
        Line::from(app.header.title.clone()),
        Line::from(app.header.status.clone()),
        Line::default(), // blank line
        Line::from(app.header.new_health_bar()),
    ]);

    let content = Paragraph::new(text).block(block);

    frame.render_widget(content, area);
}


fn draw_main(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = match app.focus {
        Focus::Main => Block::default()
            .title("Main [active]")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
        _ => Block::default().title("Main").borders(Borders::ALL),
    };

    let active_cell_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let default_cell_style = Style::default();

    let header_cells = (0..app.ts.columns)
        .map(|i| Cell::from(format!("Col {i}")));

    let header = Row::new(header_cells)
        .style(Style::default()
        .add_modifier(Modifier::BOLD));

    // -----------------------------
    // Build rows from app.table_contents
    // -----------------------------
    let selected_row = app.state.selected().unwrap_or(usize::MAX);

    let rows = app.table_contents.iter().enumerate().map(|(row_idx, row)| {
        let active_col = app.col_state.min(row.len().saturating_sub(1));

        let cells = row.iter().enumerate().map(|(col_idx, val)| {
            let mut cell = Cell::from(val.as_str()).style(default_cell_style);

            if app.focus == Focus::Main
                && row_idx == selected_row
                && col_idx == active_col
            {
                cell = cell.style(active_cell_style);
            }

            cell
        });

        Row::new(cells)
    });
  
    // -----------------------------
    // Column widths (simple default)
    // -----------------------------

    // Ratatui needs a width per column. Since you don't store widths yet,
    // just make them evenly split.
    let widths: Vec<Constraint> = (0..app.ts.columns)
        .map(|_| Constraint::Percentage((100 / app.ts.columns.max(1)) as u16))
        .collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .column_spacing(app.ts.column_spacing);

    // If you're using TableState for selection, render as stateful:
    frame.render_stateful_widget(table, area, &mut app.state);
}


fn draw_side(frame: &mut Frame, side_area: Rect, history_area: Rect, input_area: Rect, app: &mut App) {
    let inner = draw_side_border(frame, side_area, app);
    draw_side_history(frame, intersect(history_area, inner), app);
    draw_side_input(frame, intersect(input_area, inner), app);
}

// Handles focus rendering logic + border
fn draw_side_border(frame: &mut Frame, area: Rect, app: &mut App) -> Rect {
    let block = match app.focus {
        Focus::Side => Block::default()
            .title("Side [active]")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
        _ => Block::default().title("Side").borders(Borders::ALL),
    };

    let inner = block.inner(area); // return the inner area of the side_area
    frame.render_widget(block, area);
    inner
}
// List that scrolls up as you add items
fn draw_side_history(frame: &mut Frame, area: Rect, app: &mut App) {
    let list_height = area.height as usize;
    let total = app.items.len();

    let start = total.saturating_sub(list_height);
    let tail = &app.items[start..];

    let mut visible: Vec<ListItem> = Vec::new();

    let padding = list_height.saturating_sub(tail.len());
    visible.extend((0..padding).map(|_| ListItem::new("")));

    visible.extend(tail.iter().map(|s| ListItem::new(s.clone())));

    let list = List::new(visible);
    frame.render_widget(list, area);
}
// Area where user inputs items
fn draw_side_input(frame: &mut Frame, area: Rect, app: &mut App) {
    let prompt = Span::styled(
        "> ",
        Style::default().add_modifier(Modifier::BOLD),
    );

    let input_line = Line::from(vec![prompt, Span::raw(&app.input)]);

    let input = Paragraph::new(input_line);
    frame.render_widget(input, area);
}
// Helper: prevents history and input from drawing over border
fn intersect(a: Rect, b: Rect) -> Rect {
    let x1 = a.x.max(b.x);
    let y1 = a.y.max(b.y);
    let x2 = (a.x + a.width).min(b.x + b.width);
    let y2 = (a.y + a.height).min(b.y + b.height);

    Rect {
        x: x1,
        y: y1,
        width: x2.saturating_sub(x1),
        height: y2.saturating_sub(y1),
    }
}


// Popups
// ----------------------------------------------------------------------------


fn draw_game_over (frame: &mut Frame, area: Rect) {
    // make a centered rect ~40% width, 30% height of the screen
    let popup_area = centered_rect(40, 30, area);

    let block = Block::default()
        .title(" Game Over ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let text = Paragraph::new("You lose!\nPress q to quit.")
        .alignment(Alignment::Center)
        .block(block);

    // Clear what's underneath so the box looks solid
    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}


pub fn draw_help (frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(40, 30, area);

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let text = Paragraph::new("To play the Fallout hacking minigame, you must guess the correct password from a list of words, all the same length, scattered among random characters. After each guess, the Likeness score will tell you how many letters in your word are also correct and in the right position. Use this score to eliminate other possibilities from the list. You can also find and click on matching bracket pairs, such as () or <>, to either remove a wrong dud password or reset your remaining attempts. You typically have four attempts to find the correct password. If you are down to your last try, you can exit and re-enter the terminal to reset the puzzle.")
        .wrap(Wrap { trim: true})
        .scroll((1, 0))
        .block(block);

    // Clear what's underneath so the box looks solid
    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}


pub fn draw_debug (frame: &mut Frame, area: Rect, debug: &DebugLog) {
    let popup_area = centered_rect(40, 90, area);

    let block = Block::default()
        .title(" Debug ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let items: Vec<ListItem> = debug
        .iter()
        .map(|line| ListItem::new(line.as_str()))
        .collect();

    let list = List::new(items).block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(list, popup_area);
}
// helper for draw_game_over()
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // vertical split
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // horizontal split in the middle chunk
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(v[1]);

    h[1]
}
