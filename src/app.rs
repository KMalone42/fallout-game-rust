// src/app.rs

use color_eyre::Result;
use crossterm::event::{ 
    self, Event, KeyEventKind,
};
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use ratatui::layout::Rect;
use fastrand;

use crate::ui;
use crate::input::{handle_key, handle_mouse };

use std::collections::VecDeque;

// Main app loop
pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let mut side_area = Rect::default(); // updated by ui::render

    loop {
        // draw UI, passing state in
        terminal.draw(|f| ui::render(f, &mut app))?;


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

// State Handling
// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Focus {
    Head,
    Main,
    Side,
    Help,
}
impl Focus {
    pub fn focus_next_vertical(&mut self) {
        *self = match *self {
            Focus::Head => Focus::Main,               // going "down" from header
            Focus::Main | Focus::Side => Focus::Head, // from either body panel -> header
            Focus::Help => Focus::Help,
        };
    }

    pub fn focus_next_horizontal(&mut self) {
        *self = match *self {
            Focus::Head => Focus::Head,
            Focus::Main => Focus::Side,
            Focus::Side => Focus::Main,
            Focus::Help => Focus::Help,
        };
    }
}


pub struct Header {
    pub title: String,
    pub status: String,
    pub health_i: u8,
}
impl Header {
    pub fn new(starting_health: u8) -> Self {
        Self { 
            title: Self::new_title(), 
            status: Self::new_status(1),
            health_i: starting_health,
        }
    }
    // Get title & status from assets
    pub fn new_title() -> String {
        crate::assets::titles_str(0).to_string()
    }
    pub fn new_status(n: u8) -> String {
        crate::assets::status_str(n).to_string()
    }
    // Health functions
    pub fn apply_guess(&mut self, guess: &str, password: &str) -> bool {
        if guess == password {
            true
        } else {
            if self.health_i > 0 {
                self.health_i -= 1;
            }
            false
        }
    }
    pub fn is_out(&self) -> bool {
        self.health_i <= 0
    }
    /// Get the “█ █ █” bar for current health.
    pub fn new_health_bar(&self) -> &'static str{
        crate::assets::health_str(self.health_i)
    }
}


// A small structure that helps build the table in ui.rs
pub struct TableStructure{
    pub rows:    usize, // number of Row widgets to build in UI
    pub columns: usize, // ratatui::Table doesn't have a columns value, rows are actually
                        // vec![] so this will just be number of elements in that vec
    pub column_spacing: u16, // passed into column_spacing(1)

    pub hex_width: u16, // width of columns 0 & 2 need to be bound to the hex string size.
}
impl TableStructure {
    pub fn new() -> Self {
        Self { rows: 16, columns: 4, column_spacing: 1, hex_width: 1, }
    }
}

pub struct TableModel {
    pub hex_list: Vec<String>,
    pub word_list: Vec<String>,
    pub password: String,
    pub state: TableState,
}
impl TableModel {
    /// High-level constructor: this is what you call from `App::new()`.
    pub fn new(n: usize) -> Self {
        let hex_list = Self::build_hex_list();
        let word_list = Self::new_word_list(n);
        let password = Self::new_password(&word_list);
        Self { 
            hex_list,
            word_list,
            password, 
            state: TableState::new(),
        }
    }
    fn load_words() -> Vec<String> {
        let contents = std::fs::read_to_string("assets/tokens.txt")
            .expect("failed to read tokens.txt");

        contents.lines().map(|s| s.to_string()).collect()
    }
    pub fn new_word_list(n: usize) -> Vec<String> {
        let mut tokens = Self::load_words();
        fastrand::shuffle(&mut tokens);
        let word_list: Vec<String> = tokens.into_iter().take(n).collect();
        return word_list;
    }
    // Find out what the &[String] does...
    pub fn new_password(word_list: &[String]) -> String {
        let i = fastrand::usize(..word_list.len());
        let password = word_list[i].clone();
        return password;
    }

    pub fn build_hex_list() -> Vec<String> {
        let start: u32 = fastrand::u32(..);
        let count: usize = 32;
        (start..start.saturating_add(count as u32))
            .map(|n| format!("{:x}", n)) // lower-case hex; use {:X} for upper-case
            .collect()
    }
    pub fn build_alternating_lists(
        columns: usize,
        rows: usize,
        hex_list: &[String],
        word_list: &[String],
    ) -> Vec<Vec<String>> {
        assert!(columns % 2 == 0, "columns must be even");

        let mut idx1 = 0usize;
        let mut idx2 = 0usize;

        let mut result: Vec<Vec<String>> = Vec::with_capacity(rows);

        for _ in 0..rows {
            let mut inner: Vec<String> = Vec::with_capacity(columns);

            for i in 0..columns {
                if i % 2 == 0 {
                    inner.push(hex_list[idx1].clone());
                    idx1 += 1;
                } else {
                    inner.push(word_list[idx2].clone());
                    idx2 += 1;
                }
            }

            result.push(inner);
        }

        result
    }
}


pub struct DebugLog {
    lines: VecDeque<String>,
    cap: usize,
}
impl DebugLog {
    pub fn new(cap: usize) -> Self {
        Self { lines: VecDeque::with_capacity(cap), cap }
    }
    pub fn push(&mut self, s: impl Into<String>) {
        if self.lines.len() == self.cap {
            self.lines.pop_front();
        }
        self.lines.push_back(s.into());
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.lines.iter()
    }
}


pub struct App {
    pub focus: Focus,
    pub header: Header,

    pub main: TableModel,
    pub state: TableState,// row selection
    pub col_state: usize, // column selection
    pub ts: TableStructure,
    pub table_contents: Vec<Vec<String>>,

    pub items: Vec<String>,
    pub input: String,

    pub show_help: bool,
    pub game_over: bool,

    pub debug: DebugLog,
    pub show_debug: bool,

}

impl App {
    pub fn new() -> Self {
        let main = TableModel::new(32);
        let ts = TableStructure::new();
        let table_contents = TableModel::build_alternating_lists(ts.columns, ts.rows, &main.hex_list, &main.word_list);
        let state = TableState::default().with_selected(Some(0));
        let header = Header::new(4);
        Self {
            focus: Focus::Main,
            header,

            main,
            state,
            col_state: 0, // Starting column selected
            ts,
            table_contents,

            items: Vec::new(),
            input: String::new(),

            show_help: false,
            game_over: false,

            debug: DebugLog::new(200),
            show_debug: false,
        }
    }
    pub fn table_up(&mut self) {
        if self.focus != Focus::Main { return; }
        
        let len = self.table_contents.len();
        if len == 0 { return; }

        let new_idx = match self.state.selected() {
            Some(idx) if idx > 0 => idx - 1,
            Some(_) | None => len - 1,
        };
        self.state.select(Some(new_idx));
    }
    pub fn table_down(&mut self) {
        if self.focus != Focus::Main { return; }

        let len = self.table_contents.len();
        if len == 0 { return; }

        let new_idx = match self.state.selected() {
            Some(idx) if idx + 1 < len => idx + 1,
            Some(_) | None => 0,
        };
        self.state.select(Some(new_idx));
    }
    pub fn table_left(&mut self) {
        if self.focus != Focus::Main { return; }
        let max_cols = self.ts.columns.saturating_sub(1);

        if self.col_state > 0 { self.col_state -= 1; }
        else { self.col_state = max_cols; } // wrapping_sub(n)
    }
    pub fn table_right(&mut self) {
        if self.focus != Focus::Main { return; }
        let max_cols = self.ts.columns.saturating_sub(1);

        if self.col_state < max_cols { self.col_state += 1; }
        else { self.col_state = 0; } // wrapping_add(n)
    }
}
