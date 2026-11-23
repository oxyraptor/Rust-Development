use macroquad::prelude::*;
const COLS: usize = 30;
const ROWS: usize = 30;

#[macroquad::main("SnakeGame")]
async fn main() {
    let mut snake = vec![(1, 4), (1, 5)];
    let mut apple = (1, 1);
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Dirt {
        Up,
        Down,
        Left,
        Right,
    }
    impl Dirt {
        pub fn opposite(&self) -> Self {
            match self {
                Self::Up => Self::Down,
                Self::Right => Self::Left,
                Self::Down => Self::Up,
                Self::Left => Self::Right,
            }
        }
    }
    pub enum Command {
        Quit,
        Turn(Dirt),
    }

    let mut score: u16 = 0;

    loop {
        clear_background(DARKBROWN);

        let cell_size = (screen_width() / COLS as f32).min(screen_height() / ROWS as f32);
        let grid_w = cell_size * COLS as f32;
        let grid_h = cell_size * ROWS as f32;

        // Center the grid in the window
        let offset_x = (screen_width() - grid_w) / 2.0;
        let offset_y = (screen_height() - grid_h) / 2.0;

        // Draw vertical lines
        for c in 0..=COLS {
            let x = offset_x + c as f32 * cell_size;
            draw_line(x, offset_y, x, offset_y + grid_h, 1.0, GRAY);
        }

        // Draw horizontal lines
        for r in 0..=ROWS {
            let y = offset_y + r as f32 * cell_size;
            draw_line(offset_x, y, offset_x + grid_w, y, 1.0, GRAY);
        }
        draw_text("Score : 0", 20.0, 40.0, 30.0, BLACK);
        draw_rectangle(
            offset_x + apple.0 as f32 * cell_size,
            offset_y + apple.1 as f32 * cell_size,
            20.0,
            20.0,
            GREEN,
        );
        for seg in &snake {
            draw_rectangle(
                offset_x + seg.0 as f32 * cell_size,
                offset_y + seg.1 as f32 * cell_size,
                20.0,
                20.0,
                RED,
            );
        }

        KeyCode::Up) {
            Some(Command::Turn(Dirt::Up))
        }
        if is_key_pressed(KeyCode::Down) {
            Some(Command::Turn(Dirt::Down));
        }
        if is_key_pressed(KeyCode::Left) {
            Some(Command::Turn(Dirt::Left));
        }
        if is_key_pressed(KeyCode::Right) {
            Some(Command::Turn(Dirt::Right));
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
