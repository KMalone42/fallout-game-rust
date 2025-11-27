// src/state.rs

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Focus {
    Main,
    Side,
}


pub struct App {
    pub header_text: String,
    pub main_lines: Vec<String>,
    pub side_items: Vec<String>,
    pub selected: usize,
    pub focus: Focus,
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
            focus: Focus::Side, // start with sidebar focused
            // :: explanation
            // here focus variable is being assigned from the enum Focus, the side item
        }
    }
}

