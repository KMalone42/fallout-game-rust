// src/input.rs

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::Frame;
use crate::app::{App, Focus, Header, TableModel};


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
        (Char('\\'), _) => { app.show_debug = !app.show_debug; }

        // focus change with Ctrl
        (Tab, _) => { app.focus.focus_next_horizontal(); }
        (BackTab, _) => { app.focus.focus_next_horizontal(); }

        (Left,  _) => {
            app.focus.focus_next_horizontal(); 
            app.debug.push(format!("focus_left focused={:?}", app.focus));
        }
        (Right, _) => {
            app.focus.focus_next_horizontal(); 
            app.debug.push(format!("focus_right focused={:?}", app.focus));
        }
        (Up,    _) => {
            app.focus.focus_next_vertical(); 
            app.debug.push(format!("focus_up focused={:?}", app.focus));
        }
        (Down,  _) => {
            app.focus.focus_next_vertical(); 
            app.debug.push(format!("focus_down focused={:?}", app.focus));
        }

        //Char('h') if Focus::Main => { app.main.navigate_left();  }
        //Char('l') if Focus::Main => { app.main.navigate_right(); }
        (Char('j'), _) if app.focus == Focus::Main => { 
            app.table_down(); 
            app.debug.push(format!("table_down row={:?} col={}", app.state.selected(), app.col_state));
        }
        (Char('k'), _) if app.focus == Focus::Main => {
            app.table_up(); 
            app.debug.push(format!("table_up row={:?} col={}", app.state.selected(), app.col_state));
        }
        (Char('h'), _) if app.focus == Focus::Main => {
            app.table_left(); 
            app.debug.push(format!("table_left row={:?} col={}", app.state.selected(), app.col_state));
        }
        (Char('l'), _) if app.focus == Focus::Main => {
            app.table_right(); 
            app.debug.push(format!("table_right row={:?} col={}", app.state.selected(), app.col_state));
        }

        (Enter, _ ) => { 
            match app.focus { 
                Focus::Side => { 
                    if !app.input.trim().is_empty() { 
                        app.items.push(app.input.trim().to_string()); 
                        app.input.clear();
                    }
                }
                Focus::Main => { 
                    // need to get the currently hovered cell
                    let word = app.word_at_coordinates(app.col_state, app.state.selected());

                    if let Some(word) = word {
                        app.input.clear();
                        app.input.push_str(word.as_str()); // or app.input.push_str(&word);
                    }
                } 
                _ => {}
            }
        }


        //// dialog (needs to got at bottom)
        //(Char(c), m) if !m.contains(KeyModifiers::CONTROL) && !m.contains(KeyModifiers::ALT) => {
            //app.input.push(c);
        //}
        //(Backspace, _) => { app.input.pop(); }
//
        //(Enter, _) => {
            //if !app.input.trim().is_empty() {
                //app.items.push(app.input.trim().to_string());
            //}
            //app.input.clear();
        //}
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
        }
        _ => {}
    }
}
