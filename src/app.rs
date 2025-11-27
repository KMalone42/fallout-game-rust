// src/app.rs

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, KeyEvent, MouseEventKind};
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
            Event::Key(key) => handle_key(key, &mut app),
            Event::Mouse(me) => handle_mouse()
        }



        fn handle_key(key: KeyEvent, app: &mut App) {
            use KeyCode::*;
            let code = key.code;
            let mods = key.modifiers;

            match (code, mods) {
                // quit
                KeyCode::Char('q') => { break Ok(()); }


                // focus change
                KeyCode::Tab | KeyCode::Right => { focus_next(app) }
                KeyCode::BackTab | KeyCode::Left => { focus_prev(app) }

                (Left,  m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_prev_pane(app);
                }
                (Right, m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_next_pane(app);
                }
                (Up,    m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_prev_pane(app);                 
                }
                (Down,  m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_next_pane(app);
                }

                (KeyCode::Char('h'), m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_left(app) 
                }
                (KeyCode::Char('j'), m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_down(app) 
                }
                (KeyCode::Char('k'), m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_up(app) 
                }
                (KeyCode::Char('l'), m) if m.contains(KeyModifiers::SHIFT) => {
                    focus_right(app) 
                }

                // navigation in current focus

                (Up,    _) => navigate_up(app),
                (Down,  _) => navigate_down(app),
                (Left,  _) => navigate_left(app),
                (Right, _) => navigate_right(app),

                (KeyCode::Char('h'), _) => navigate_left(app),
                (KeyCode::Char('j'), _) => navigate_down(app),
                (KeyCode::Char('k'), _) => navigate_up(app),
                (KeyCode::Char('l'), _) => navigate_right(app),

                // ------------------------------------------------------------
                _ => {}
            }
        };


        // Todo, get this working
        fn handle_mouse(app: &mut App, side_area: Rect) {
        }
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                // adding this match structure is kind of like a switch statement
                // better than the old 'if' statement we had


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

