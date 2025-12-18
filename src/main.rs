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

//fn main() -> Result<()> {
    //color_eyre::install()?;
    //enable_raw_mode()?;
    //let mut stdout = stdout();
    //execute!(stdout, EnterAlternateScreen)?;
//
    //let backend = CrosstermBackend::new(stdout);
    //let mut terminal = Terminal::new(backend)?;
//
    //let res = run(&mut terminal);
//
    //disable_raw_mode()?; // also not sure what this was doing
    //execute!(io::stdout(), LeaveAlternateScreen)?; // not sure what the AlternateScreen function is
    //// doing
    //terminal.show_cursor()?;
//
    //res
//}
