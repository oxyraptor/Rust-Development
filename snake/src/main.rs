use macroquad::prelude::*;
use std::collections::VecDeque;

const COLS: usize = 30;
const ROWS: usize = 30;

#[macroquad::main("SnakeGame")]
async fn main() {
    let mut snake: Vec<(i32, i32)> = vec![(1, 4), (1, 5)];
    let mut apple: (i32, i32) = (1, 1);
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

    let mut game_over: bool = false;

    let mut input_queue = VecDeque::new();

    loop {
        clear_background(DARKBROWN);

        let cell_size = (screen_width() / COLS as f32).min(screen_height() / ROWS as f32);
        let grid_w = cell_size * COLS as f32;
        let grid_h = cell_size * ROWS as f32;

        // Center the grid in the window
        let offset_x = (screen_width() - grid_w) / 2.0;
        let offset_y = (screen_height() - grid_h) / 2.0;

        draw_text(&format!("Score: {}", score), 20.0, 40.0, 30.0, BLACK);
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

        if input_queue.len() < 2 {
            let last_dir = input_queue.back().copied().unwrap_or(dir);
            if is_key_pressed(KeyCode::Up) {
                let new_dir = Dirt::Up;
                if new_dir != last_dir && new_dir != last_dir.opposite() {
                    input_queue.push_back(new_dir);
                }
            } else if is_key_pressed(KeyCode::Down) {
                let new_dir = Dirt::Down;
                if new_dir != last_dir && new_dir != last_dir.opposite() {
                    input_queue.push_back(new_dir);
                }
            } else if is_key_pressed(KeyCode::Left) {
                let new_dir = Dirt::Left;
                if new_dir != last_dir && new_dir != last_dir.opposite() {
                    input_queue.push_back(new_dir);
                }
            } else if is_key_pressed(KeyCode::Right) {
                let new_dir = Dirt::Right;
                if new_dir != last_dir && new_dir != last_dir.opposite() {
                    input_queue.push_back(new_dir);
                }
            }

            if is_key_pressed(KeyCode::Escape) {
                break;
            }
        }

        if !game_over {
            if get_time() - last_updated > speed {
                last_updated = get_time();
                let mut new_head = snake[0];
                if let Some(next_dir) = input_queue.pop_front() {
                    dir = next_dir;
                }
                match dir {
                    Dirt::Up => new_head.1 -= 1,
                    Dirt::Down => new_head.1 += 1,
                    Dirt::Left => new_head.0 -= 1,
                    Dirt::Right => new_head.0 += 1,
                }

                new_head.0 = new_head.0.rem_euclid(COLS as i32);
                new_head.1 = new_head.1.rem_euclid(ROWS as i32);
                if snake.contains(&new_head) {
                    game_over = true;
                } else {
                    snake.insert(0, new_head);
                    if new_head == apple {
                        score += 1;
                        apple = (rand::gen_range(0, 30), rand::gen_range(0, 30));
                    } else {
                        snake.pop();
                    }
                }
            }
        }

        if game_over {
            draw_text(
                "Game Over cuz",
                screen_width() / 2.0,
                screen_height() / 2.0,
                60.0,
                RED,
            );
        }

        next_frame().await
    }
}
