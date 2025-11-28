use std::io::{self, stdout};
use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Frame;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

struct App {
    items: Vec<String>,
    input: String,
}

impl App {
    fn new() -> Self {
        Self {
            items: vec![
                "First item".to_string(),
                "Second item".to_string(),
                "Third item".to_string(),
            ],
            input: String::new(),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Ignore key-release / repeat noise in some terminals
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    // Quit with q or Esc when input is empty
                    KeyCode::Char('q') if app.input.is_empty() => return Ok(()),
                    KeyCode::Esc if app.input.is_empty() => return Ok(()),

                    // Typing into the dialog
                    KeyCode::Char(c) => app.input.push(c),

                    // Backspace in the dialog
                    KeyCode::Backspace => {
                        app.input.pop();
                    }

                    // Add to list on Enter
                    KeyCode::Enter => {
                        if !app.input.trim().is_empty() {
                            app.items.push(app.input.trim().to_string());
                        }
                        app.input.clear();
                    }

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}


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

    // Input area (unchanged)
    let prompt = Span::styled(
        "Add item: ",
        Style::default().add_modifier(Modifier::BOLD),
    );
    let input_line = Line::from(vec![prompt, Span::raw(&app.input)]);
    let input = Paragraph::new(input_line);
    f.render_widget(input, chunks[1]);
}

