// src/app.rs

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;

use crate::state::{App, Focus};
use crate::ui;

/// Main app loop.
pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let mut side_area = Rect::default(); // updated by ui::render

    loop {
        // draw UI, passing state in
        terminal.draw(|f| ui::render(f, &app, &mut side_area))?;

        match event::read()? {
            Event::Key(key) => {
                // adding this match structure is kind of like a switch statement
                // better than the old 'if' statement we had
                match key.code {
                    KeyCode::Char('q') => {
                        break Ok(());
                    }
                    // focus movement between panes
                    KeyCode::Tab | KeyCode::Right => {
                        app.focus = Focus::Side;
                    }
                    KeyCode::BackTab | KeyCode::Left => {
                        app.focus = Focus::Main;
                    }

                    // navigate the list only when sidebar is focused
                    KeyCode::Up => {
                        if app.focus == Focus::Side && app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.focus == Focus::Side
                            && app.selected + 1 < app.side_items.len()
                        {
                            app.selected += 1;
                        }
                    }
                    _ => {} // this is called a 'catch-all match arm'
                           // prevents tui from breaking on undefined key press
                }
            }
            // this is rediculous
            Event::Mouse(me) => {
                match me.kind {
                    MouseEventKind::Moved | MouseEventKind::Down(_) => {
                        let x = me.column;
                        let y = me.row;

                        // is mouse over sidebar?
                        if x >= side_area.x
                            && x < side_area.x + side_area.width
                            && y >= side_area.y
                            && y < side_area.y + side_area.height
                        {
                            // inside border
                            let inner_top = side_area.y + 1;
                            if y >= inner_top {
                                let row_index = (y - inner_top) as usize;
                                let max_index =
                                    app.side_items.len().saturating_sub(1);
                                app.selected = row_index.min(max_index);
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

