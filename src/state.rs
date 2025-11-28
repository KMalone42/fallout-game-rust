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

pub struct Health {
    pub fn new(start: i32) -> Self {
        Self { current: start }
    }
}

impl Health {
    // On guess if incorrect() 
    pub fn apply_guess(&mut self, guess: &str, password: &str) -> bool {
        if guess == password {
            true
        }
    } else {
        if self.current > 0 {
            self.current -= 1;
        }
        false
    }

    pub fn is_out(&self) -> bool {
        self.current <= 0
    }
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
            // :: explanation
            // here focus variable is being assigned from the enum Focus, the side item
        }
    }
}

