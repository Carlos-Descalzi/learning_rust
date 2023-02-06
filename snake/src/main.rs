mod game;
use termion::raw::IntoRawMode;
use std::io::stdout;
use std::io::Read;

fn main() {

    let mut stdout = stdout().lock().into_raw_mode().unwrap();
    let mut reader = termion::async_stdin().bytes();
    let mut terminal = game::Terminal::new(& mut stdout, & mut reader);

    game::run(&mut terminal);

}
