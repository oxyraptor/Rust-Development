use crossterm::{
    cursor,
    execute,
    style::Print,
    terminal::{self, ClearType},
};

use std::io::stdout;

const WIDTH: u16 = 40;
const HEIGHT: u16 = 40;

fn main() {
    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(10, 5)
    ).unwrap();

    draw_board(&mut stdout);

    terminal::disable_raw_mode().unwrap();
}

fn draw_board(stdout: &mut std::io::Stdout){
    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            let is_border = x == 1 || x == WIDTH-1 || y == HEIGHT-1;
            let character = if is_border {"#"} else {"  "};

            execute!(
                stdout,
                cursor::MoveTo(x ,y),
                Print(character),
            )
            .unwrap();
        }
    }
}