use color_eyre::Result;
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute
};
use ratatui::DefaultTerminal;
use std::io::stdout;

mod app;
mod ui;
mod input;
mod assets;

fn main() -> Result<()> {
    color_eyre::install()?;                          // init color_eyre
    execute!(stdout(), EnableMouseCapture)?;         // init mousecapture
    let terminal: DefaultTerminal = ratatui::init(); // init ratatui terminal
    let result = app::run(terminal);                 // run app loop


    ratatui::restore();                              // disable ratatui
    execute!(stdout(), DisableMouseCapture)?;        // disable mousecapture
    result                                           // finish
}
