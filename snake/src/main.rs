use macroquad::prelude::*;

// We define our grid size.
const GRID_WIDTH: i16 = 40;
const GRID_HEIGHT: i16 = 20;
// How many pixels wide/tall each "block" of the snake is.
const CELL_SIZE: f32 = 20.0; 

// Define the directions the snake can move
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}