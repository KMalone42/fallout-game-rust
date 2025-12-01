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
