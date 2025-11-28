// src/app.rs

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, KeyEvent, MouseEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;

use crate::state::{App, Focus};
use crate::ui;

use input


/// Main app loop.
pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let mut side_area = Rect::default(); // updated by ui::render

    loop {
        // draw UI, passing state in
        terminal.draw(|f| ui::render(f, &app, &mut side_area))?;

        match event::read()? {
            Event::Key(key) => handle_key(key, &mut app),
            Event::Mouse(me) => handle_mouse()
        }

        // Ignore key-release / repeat noise in some terminals
        if key.kind != KeyEventKind::Press {
            continue;
        }
    }
}
