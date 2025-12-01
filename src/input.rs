// src/input.rs

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::state::{App, Focus};


// Return 'true' if the app should quit, 'false' otherwise.
//
// i don't really know how much i agree with -> bool
pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    use KeyCode::*;
    let code = key.code;
    let mods = key.modifiers;

    match (code, mods) {
        // quit
        (Char('q'), _) => { return true; } // signal quit

        (Char('?'), _) => { app.show_help = !app.show_help; }

        // focus change with Ctrl
        (Tab, _) => { app.focus.focus_next_horizontal(); }
        (BackTab, _) => { app.focus.focus_next_horizontal(); }
        (Right, m) if m.contains(KeyModifiers::CONTROL) => { app.focus.focus_next_horizontal(); }
        (Left, m) if m.contains(KeyModifiers::CONTROL) => { app.focus.focus_next_horizontal(); }

         // shift + direction = focus movement
        (Left,  m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_horizontal(); }
        (Right, m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_horizontal(); }
        (Up,    m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_vertical(); }
        (Down,  m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_vertical(); }

        (Char('h'), m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_horizontal(); }
        (Char('l'), m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_horizontal(); }
        (Char('j'), m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_vertical(); }
        (Char('k'), m) if m.contains(KeyModifiers::SHIFT) => { app.focus.focus_next_vertical(); }

        // navigation in current focus
        (Up,    _) => navigate_up(app),
        (Down,  _) => navigate_down(app),
        (Left,  _) => navigate_left(app),
        (Right, _) => navigate_right(app),

        (Char('h'), _) => navigate_left(app),
        (Char('j'), _) => navigate_down(app),
        (Char('k'), _) => navigate_up(app),
        (Char('l'), _) => navigate_right(app),

        // dialog (needs to got at bottom)
        (Char(c), m) if !m.contains(KeyModifiers::CONTROL) && !m.contains(KeyModifiers::ALT) => {
            app.input.push(c);
        }
        (Backspace, _) => { app.input.pop(); }

        (Enter, _) => {
            if !app.input.trim().is_empty() {
                app.items.push(app.input.trim().to_string());
            }
            app.input.clear();
        }
        

        _ => {}
    }

    false
}


// Mouse handler takes a MouseEvent, not Event::Mouse
pub fn handle_mouse(me: MouseEvent, app: &mut App, side_area: Rect) {
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
                // inside border (skip top border row)
                let inner_top = side_area.y + 1;
                if y >= inner_top {
                    let row_index = (y - inner_top) as usize;
                    let max_index = app.side_items.len().saturating_sub(1);
                    app.selected = row_index.min(max_index);
                }
            }
        }
        _ => {}
    }
}

// These would live here too or in another module; just stubbed for now:
fn navigate_up(app: &mut App)   { /* ... */ }
fn navigate_down(app: &mut App) { /* ... */ }
fn navigate_left(app: &mut App) { /* ... */ }
fn navigate_right(app: &mut App){ /* ... */ }
