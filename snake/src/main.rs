use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};

use std::io::stdout;

fn main() {
    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(10, 5)
    ).unwrap();

    println!("HELLO FROM RUST SNAKE!");

    terminal::disable_raw_mode().unwrap();
}