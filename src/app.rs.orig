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

pub struct PlaySpace {
    pub chars: Vec<char>,
    pub char_cells: Vec<String>,
}

// Strings of special characters which yield special modifiers
#[derive(Debug)]
pub struct ModifierString {
    pub open: char,
    pub close: char,
    pub open_idx: usize,
    pub close_idx: usize,
    pub inbetween: String,
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
<<<<<<< HEAD
    pub junk_words: (Vec<String>, Vec<usize>),
    pub play_space: (Vec<char>, Vec<String>),
=======
    pub junk_word_list: Vec<String>,
    pub play_space: PlaySpace,
>>>>>>> a76a2a6 (playspace object)
    pub password: String,
    pub state: TableState,
}
impl TableModel {
    /// High-level constructor: this is what you call from `App::new()`.
    pub fn new() -> Self {
        let hex_list = Self::build_hex_list();
        let word_list = Self::new_word_list(8);
        let password = Self::new_password(&word_list);
        let junk_words = Self::generate_junk(&word_list);
        let play_space = Self::build_play_space(&junk_words.0);
        Self { 
            hex_list,
            word_list,
            password, 
            junk_words,
            play_space,
            state: TableState::new(),
        }
    }

    pub fn build_hex_list() -> Vec<String> {
        let start: u32 = fastrand::u32(..);
        let count: usize = 32;
        (start..start.saturating_add(count as u32))
            .map(|n| format!("{:x}", n)) // lower-case hex; use {:X} for upper-case
            .collect()
    }

    /* Chain for generating a word list, takes tokens from assets/tokens.txt 
    *  generates word_list -> gets password from list
    */
    fn load_words() -> Vec<String> {
        let contents = std::fs::read_to_string("assets/tokens.txt")
            .expect("failed to read tokens.txt");

        contents.lines().map(|s| s.to_string()).collect()
    }
    // n = number of words in play area
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

    /* balls n bins implementation generates junk around each word and returns a 
    *  Vec<String> that looks like (junk, word, junk, word...
    */
    pub fn generate_junk(word_list: &[String]) -> (Vec<String>, Vec<usize>) {
        let total_chars: usize = word_list.iter().map(|s| s.chars().count()).sum();
        let total_junk: usize = 256usize.saturating_sub(total_chars);

        let (mut content, empty_indices): (Vec<String>, Vec<usize>) = word_list
            .into_iter()
            .flat_map(|w| [String::new(), w.clone()])  // build "", w, "", w, ...
            .enumerate()                               // attach final index: (idx, item)
            .fold((Vec::new(), Vec::new()), |(mut out, mut idxs), (i, s)| {
                if s.is_empty() {                      // the inserted ""
                    idxs.push(i);
                }
                out.push(s);
                (out, idxs)
            });

        assert!(!empty_indices.is_empty(), "nowhere to put junk (no bins)");
        if total_junk == 0 { return (content, empty_indices); }

        let fr_max_1: usize = empty_indices.len();
        let junkpool: Vec<char> =
            "!@#$%^&*(){}[]<>+-_=|\\/:;'\"`,.?~".chars().collect();
        let fr_max_2: usize = junkpool.len();

        // get a random index empty_indices[i] to place a random char junkpool[j] into
        // content[empty_indices[i]]
        for _ in 0..total_junk { 
            let i = fastrand::usize(..fr_max_1);
            let j = fastrand::usize(..fr_max_2);

            let junk_char = junkpool[j];

            content[empty_indices[i]].push(junk_char);
        };
        return (content, empty_indices);
    }

<<<<<<< HEAD
    /* helper function turns output of generate_junk to a string then back into a
    *  vec where each cell is an equal number of characters.
    */
    pub fn build_play_space(junk_word_list: &[String]) -> (Vec<char>, Vec<String>) {
        let cell_len = 8;
        let total_cells = 32;

        // Flatten into Vec<char> so we can chunk by *characters* safely
        let chars: Vec<char> = junk_word_list.iter().flat_map(|s| s.chars()).collect();

        assert_eq!(chars.len(), cell_len * total_cells, "expected 256 chars total");

        // Chunk into 8-char cells
        let chunked_chars = chars
            .chunks(cell_len)
            .take(total_cells)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        return (chars, chunked_chars);
    }


    /* returns substring between FIRST open and LAST close (exclusive)
    *  helper function for eval_play_space 
    */ 
    pub fn capture_between(s: &str, open: char, close: char) -> Option<String> {
        let start = s.find(open)?;     // byte index of first open
        let end   = s.rfind(close)?;   // byte index of last close
        if end <= start { return None; }

        // we want *inside* the delimiters
        let inside = &s[start + open.len_utf8() .. end];
        Some(inside.to_string())
    }
    /* returns a list of valid playable strings in the existing play space
    *  take the output of generate junk and evaluate all the symbol strings
    */
    pub fn eval_play_space(mut junk_indices: Vec<usize>, content: Vec<String>) -> Vec<String> {
        // junk_indices from generate_junk.empty_indices is used as a map to tell us where junk was likely 
        // placed within content before processing these things however we 
        // need to find out if any of these were left empty
        junk_indices.retain(|&i| !content[i].is_empty());

        // captures of text *between* first opener and last closer
        let mut captures: Vec<String> = Vec::new();

        for &idx in &junk_indices {
            let junk: &str = content[idx].as_str();

            // () [] {} <> "" ''
            if let Some(s) = Self::capture_between(junk, '(', ')') { captures.push(s); }
            if let Some(s) = Self::capture_between(junk, '[', ']') { captures.push(s); }
            if let Some(s) = Self::capture_between(junk, '{', '}') { captures.push(s); }
            if let Some(s) = Self::capture_between(junk, '<', '>') { captures.push(s); }
            if let Some(s) = Self::capture_between(junk, '"', '"') { captures.push(s); }
            if let Some(s) = Self::capture_between(junk, '\'', '\'') { captures.push(s); }
        }

        // take captures and words from content
        content
    }
=======
>>>>>>> a76a2a6 (playspace object)


    /* builds a Vec<Vec<String>> that looks likes this
    *  hex play[0] hex play[2]
    *  hex play[1] hex play[3]...
    */
    pub fn build_alternating_lists(
        columns: usize,
        rows: usize,
        hex_list: &[String],
        play_space: &[String],
    ) -> Vec<Vec<String>> {
        assert!(columns == 4,           "there should be 4 columns");
        assert!(rows == 16,             "there should be 16 rows for play_space");
        assert!(play_space.len() == 32, "junk+word_list should be split into 32 cells");
        assert!(hex_list.len() == 32,   "hex_list should have 32 elements");

        let mut hex_idx = 0usize; // global iterator for hex_list


        let mut result: Vec<Vec<String>> = Vec::with_capacity(rows);

        for i in 0..rows {
            let mut inner: Vec<String> = Vec::with_capacity(columns);

            for j in 0..columns {
                if j % 2 == 0 {
                    inner.push(hex_list[hex_idx].clone());
                    hex_idx += 1;
                } else {
                    // compute play_idx if % 2 return value for first column else second column
                    // col 1 & 3 should be getting values from play_space
                    let odd_k = j / 2;
                    let idx = i + odd_k * 16;
                    inner.push(play_space[idx].clone());
                }
            }

            result.push(inner);
        }

        result
    }

    /* helper function turns output of generate_junk to a string then back into a
    *  vec where each cell is an equal number of characters.
    */
    pub fn build_play_space(junk_word_list: &[String]) -> PlaySpace {
        let cell_len = 8;
        let total_cells = 32;

        // Flatten into Vec<char> so we can chunk by *characters* safely
        let chars: Vec<char> = junk_word_list.iter().flat_map(|s| s.chars()).collect();

        assert_eq!(chars.len(), cell_len * total_cells, "expected 256 chars total");

        // Chunk into 8-char cells
        let char_cells: Vec<String> = chars
            .chunks(cell_len)
            .take(total_cells)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        PlaySpace { chars, char_cells }
    }

    /* returns a list of valid playable strings in the existing play space
    */
    pub fn capture_bracket_pairs(input: &str) -> Result<Vec<ModifierString>, String> {
        let mut stack: Vec<(char, usize)> = Vec::new();
        let mut pairs: Vec<ModifierString> = Vec::new();

        for (idx, ch) in input.chars().enumerate() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push((ch, idx));
                }
                ')' | ']' | '}' | '>' => {
                    let (open, open_idx) = stack.pop()
                        .ok_or_else(|| format!("Unmatched closing '{}' at {}", ch, idx))?;

                    if !is_matching_pair(open, ch) {
                        return Err(format!(
                            "Mismatched pair: '{}' at {} does not match '{}' at {}",
                            open, open_idx, ch, idx
                        ));
                    }

                    pairs.push(ModifierString {
                        open,
                        close: ch,
                        open_idx,
                        close_idx: idx,
                    });
                }
                _ => {}
            }
        }

        if let Some((open, idx)) = stack.pop() {
            return Err(format!("Unmatched opening '{}' at {}", open, idx));
        }

        Ok(pairs)
    }

    fn is_matching_pair(open: char, close: char) -> bool {
        matches!(
            (open, close),
            ('(', ')') |
            ('[', ']') |
            ('{', '}') |
            ('<', '>')
        )
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
    pub fn len(&self) -> usize { self.lines.len() }
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
        let main = TableModel::new();
        let ts = TableStructure::new();
        let table_contents = TableModel::build_alternating_lists(ts.columns, ts.rows, &main.hex_list, &main.play_space.1);
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

    pub fn word_at_coordinates(&mut self, x: usize, y: Option<usize>) -> Option<String> {
        let y = y?; // unwraps Option<usize> into usize
        self.debug.push(format!("word_at_coordinates row={y} col={x}"));
        self.table_contents.get(y)
         .and_then(|row| row.get(x))
         .cloned()
    }
}
