mod app;
mod popup_factory;
mod popups;
mod screens;
mod storage;
mod theme;
mod utils;
use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
