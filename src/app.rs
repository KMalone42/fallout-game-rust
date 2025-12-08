// src/app.rs

use color_eyre::Result;
use crossterm::event::{ 
    self, Event, KeyCode, KeyModifiers, KeyEvent, KeyEventKind, MouseEventKind,
};
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;

use crate::state::{App, Focus};
use crate::ui;

use crate::input::{handle_key, handle_mouse };


// Main app loop
pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let mut side_area = Rect::default(); // updated by ui::render

    loop {
        // draw UI, passing state in
        terminal.draw(|f| ui::render(f, &app, &mut side_area))?;

        match event::read()? {
            Event::Key(key) => {
                // only on key press (skip repeats / releases)
                if key.kind == KeyEventKind::Press {
                    if handle_key(key, &mut app) {
                        // handle_key returns true = quit
                        return Ok(());
                    }
                }
            }
            Event::Mouse(me) => {
                handle_mouse(me, &mut app, side_area);
            }
            _ => {}
        }
    }
}




// src/state.rs
use crossterm::terminal::supports_keyboard_enhancement;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Focus {
    Head,
    Main,
    Side,
}


impl Focus {
    /// Move focus vertically (Up/Down).
    /// Head <-> (Main/Side row)
    pub fn focus_next_vertical(self) -> Self {
        match self {
            Focus::Head => Focus::Main,          // going "down" from header
            Focus::Main | Focus::Side => Focus::Head, // from either body panel -> header
        }
    }

    /// Move focus horizontally (Left/Right).
    /// Only switches between Main and Side.
    pub fn focus_next_horizontal(self) -> Self {
        match self {
            Focus::Head => Focus::Head, // header doesn't move horizontally
            Focus::Main => Focus::Side,
            Focus::Side => Focus::Main,
        }
    }
}

pub struct Title {

}

pub struct Health {
    pub current: i32,
}

impl Health {
    pub fn new(start: i32) -> Self {
        Self { current: start }
    }

    // On guess if incorrect() 
    pub fn apply_guess(&mut self, guess: &str, password: &str) -> bool {
        if guess == password {
            true
        } else {
            if self.current > 0 {
                self.current -= 1;
            }
            false
        }
    }

    pub fn is_out(&self) -> bool {
        self.current <= 0
    }
}

pub struct Table {
    pub content: String,
}

impl Table {
    // Generate the game board
    pub fn new(start: String) -> Self {

    }
    // playable
    // obtain list of words from 
    fn get_words() -> arr {

    }
    std::fs::read_to_lines()
}

pub struct App {
    pub header_text: String,
    pub main_lines: Vec<String>,
    pub side_items: Vec<String>,
    pub selected: usize,
    pub focus: Focus,

    pub table_col: usize,
    pub table_row: usize,

    pub health: Health,
    pub game_over: bool,

    pub input: String,
    pub items: Vec<String>,

    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            header_text: "My Ratatui App".into(),
            main_lines: vec!["line 1".into(), "line 2".into()],
            side_items: vec![
                "Item 1".into(),
                "Item 2".into(),
                "Item 3".into(),
            ],
            selected: 0,
            focus: Focus::Side,
            table_col: 1,
            table_row: 1,
            health: Health::new(4),
            game_over: false,
            input: String::new(),
            items: Vec::new(),
            show_help: false,
        }
    }
}




// src/list-withdialogue.rs
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
