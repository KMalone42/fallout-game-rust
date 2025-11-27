use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use ratatui::{prelude::*, widgets::*};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

fn main() -> Result<()> {
    use std::io::stdout;
    use crossterm::execute;
    use crossterm::event::EnableMouseCapture;

    color_eyre::install()?;
    execute!(stdout(), EnableMouseCapture)?; // track user mouse
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, MouseEventKind};
    use ratatui::layout::Rect;

    let mut app = App::new();            // state (unused for now, fine)
    let mut selected = 0usize;           // which sidebar item is highlighted
    let mut side_area = Rect::default(); // will be filled by render()

    loop {
        // draw, telling render what is selected, and letting it update side_area
        terminal.draw(|f| render(f, selected, &mut side_area))?;

        match event::read()? {
            Event::Key(key) => {
                if let KeyCode::Char('q') = key.code {
                    break Ok(());
                }
            }
            Event::Mouse(me) => {
                use crossterm::event::MouseEventKind;
                match me.kind {
                    MouseEventKind::Moved | MouseEventKind::Down(_) => {
                        let x = me.column;
                        let y = me.row;

                        if x >= side_area.x
                            && x < side_area.x + side_area.width
                            && y >= side_area.y
                            && y < side_area.y + side_area.height
                        {
                            let inner_top = side_area.y + 1;
                            if y >= inner_top {
                                let row_index = (y - inner_top) as usize;
                                let max_index = 3usize.saturating_sub(1); // 3 items
                                selected = row_index.min(max_index);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, selected: usize, side_area_out: &mut Rect) {

    // frame.area() is a struct that looks like
    // ( x, y, width, height )
    let root = frame.area(); // whole terminal
    let mut ui = root;

    // logic for different terminal sizes (simulates 4:3 aspect ratio)
    // really an 80:24 is simplified to 10:3
    if ui.height * 10/3 <= ui.width {
        ui.width = ui.height * 10/3; // width shrinks to fit height
    } else {
        ui.height = ui.width * 3/10; // height shrinks to fit width
    }

    // Vertical split: header (fixed height) + body (rest)
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header height
            Constraint::Min(0),     // body gets the rest
        ])
        .split(ui); // use the bounds created by ui logic from before

    let header_area = vertical[0]; // full-width header
    let body_area   = vertical[1]; // full-width body

    // Inside body: main + sidebar, dynamic width by percentage
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(body_area);

    let main_area = body_chunks[0];
    let side_area = body_chunks[1];

    // store sidebar rect so run() can use it
    *side_area_out = side_area;

    // Render blocks and their contents
    draw_header(frame, header_area);
    draw_main(frame, main_area);
    let selected = 0;
    draw_side(frame, side_area, selected);
}

fn draw_header(frame: &mut Frame, area: Rect) {
    let block = Block::default().title("HEADER").borders(Borders::ALL);

    let content = Paragraph::new("Header content goes here")
        .block(block); // ← attach block to content

    frame.render_widget(content, area);
}

fn draw_main(frame: &mut Frame, area: Rect) {
    let block = Block::default().title("MAIN").borders(Borders::ALL);

    let content = Paragraph::new("Main content goes here")
        .block(block); // ← attached here

    frame.render_widget(content, area);
}

fn draw_side(frame: &mut Frame, area: Rect, selected: usize) {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];

    let block = Block::default().title("SIDE").borders(Borders::ALL);

    let list = List::new(items)
        .block(block) // ← attach border
        .highlight_symbol("➤ ")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
        );
    // selection state
    let mut state = ListState::default();
    state.select(Some(selected));

    frame.render_stateful_widget(list, area, &mut state);
}

// define state
struct App {
    header_text: String,
    main_lines: Vec<String>,
    side_items: Vec<String>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            header_text: "My Ratatui App".into(),
            main_lines: vec!["line 1".into(), "line 2".into()],
            side_items: vec!["Item A".into(), "Item B".into()],
            selected: 0,
        }
    }
}

