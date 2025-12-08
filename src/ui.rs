use ratatui::prelude::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph,
    Table, Row, Cell, Clear, Wrap,
};
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style};

use crate::app::{App, Focus};

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

    if app.game_over {
        draw_game_over(frame, root);
    }

    if app.show_help {
        let area = frame.size();
        draw_help(frame, area);
    }




    // Input area (unchanged)
    let prompt = Span::styled(
        "Add item: ",
        Style::default().add_modifier(Modifier::BOLD),
    );
    let input_line = Line::from(vec![prompt, Span::raw(&app.input)]);
    let input = Paragraph::new(input_line);
    f.render_widget(input, chunks[1]);
    // ------------------------------------------------------------------------
    // Not placed correctly but i can figure that this area should be in this function
    //
    //
    //

    // boiler likely unneeded
fn ui(f: &mut Frame, app: &App) {
    let area = f.area();

    let outer = Block::default().borders(Borders::ALL).title("Items");
    f.render_widget(outer.clone(), area);

    let inner = outer.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),   // list
                Constraint::Length(3) // input
            ]
            .as_ref(),
        )
        .split(inner);


}
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


    let active_cell_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let default_cell_style = Style::default();

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
    // build rows
    // ---------------------------------------------------------------------
    let rows = (0..16).map(|row_idx| {
        let c0 = Cell::from(format!("r{row_idx}c0")).style(default_cell_style);

        let mut c1 = Cell::from(format!("row {row_idx} wide text col1"));
        let mut c2 = Cell::from(format!("c2-{row_idx}"));
        let mut c3 = Cell::from(format!("wide col3 text for row {row_idx}"));

        // if MAIN is focused *and* this is the selected row,
        // apply active style to the active column
        if app.focus == Focus::Main && row_idx == app.selected {
            match app.table_col {
                1 => {
                    c1 = c1.style(active_cell_style);
                }
                3 => {
                    c3 = c3.style(active_cell_style);
                }
                _ => {}
            }
        }

        Row::new(vec![c0, c1, c2, c3])
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

    // -------- BOTTOM-ALIGNED, ENDLESS LIST VIEW --------
    let list_area = chunks[0];
    let list_height = list_area.height as usize;

    let total = app.items.len();

    // Take only the last `list_height` items (tail)
    let start = total.saturating_sub(list_height);
    let tail = &app.items[start..];

    let mut visible_items: Vec<ListItem> = Vec::new();

    // Pad on top so the tail hugs the bottom
    let padding = list_height.saturating_sub(tail.len());
    for _ in 0..padding {
        visible_items.push(ListItem::new(""));
    }

    for text in tail {
        visible_items.push(ListItem::new(text.clone()));
    }

    let list = List::new(visible_items);
    f.render_widget(list, list_area);
    // -------------------------------------
    // Not placed correctly




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
