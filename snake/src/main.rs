use macroquad::prelude::*;
const COLS: usize = 30;
const ROWS: usize = 30;

#[macroquad::main("SnakeGame")]
async fn main() {
    let mut snake = vec![1, 2];
    let mut apple = (1, 1);
    enum Dirt {
        Up,
        Down,
        Left,
        Right,
    }

    const CELL_SIZE: f32 = 20.0;

    let mut score: u16 = 0;

    loop {
        clear_background(DARKBROWN);
        draw_rectangle(240.0, 300.0, 20.0, 20.0, RED);
        draw_rectangle(25.0, 60.0, 20.0, 20.0, GREEN);

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

        next_frame().await
    }
}
