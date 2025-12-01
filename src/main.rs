use color_eyre::Result;
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute
};
use ratatui::DefaultTerminal;
use std::io::stdout;

mod app;
mod state;
mod ui;
mod input;

fn main() -> Result<()> {
    color_eyre::install()?;

    // enable mouse tracking
    execute!(stdout(), EnableMouseCapture)?;

    // init ratatui terminal
    let terminal: DefaultTerminal = ratatui::init();

    // run app loop
    let result = app::run(terminal);

    // restore terminal & mouse state
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;
    result
}

