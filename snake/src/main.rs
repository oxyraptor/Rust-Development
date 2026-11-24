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
    let mut dir = Dirt::Right;
    let mut last_updated = get_time();
    let mut speed: f64 = 0.15;

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

        if is_key_pressed(KeyCode::Up) {
            if (dir != Dirt::Up.opposite()) {
                dir = Dirt::Up;
            }
        }
        if is_key_pressed(KeyCode::Down) {
            if (dir != Dirt::Down.opposite()) {
                dir = Dirt::Down;
            }
        }
        if is_key_pressed(KeyCode::Left) {
            if (dir != Dirt::Left.opposite()) {
                dir = Dirt::Left;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if (dir != Dirt::Right.opposite()) {
                dir = Dirt::Right;
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if get_time() - last_updated > speed {
            last_updated = get_time();
            let mut new_head = snake[0];
            match dir {
                Dirt::Up => new_head.1 -= 1,
                Dirt::Down => new_head.1 += 1,
                Dirt::Left => new_head.0 -= 1,
                Dirt::Right => new_head.0 += 1,
            }
            snake.insert(0, new_head);
            snake.pop();
        }

        next_frame().await
    }
}
