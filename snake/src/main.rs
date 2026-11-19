use macroquad::prelude::*;

#[macroquad::main("SnakeGame")]
async fn main(){
    
    loop{
        clear_background(DARKBROWN);
        draw_rectangle(250.0, 300.0, 20.0, 20.0, RED);
        draw_rectangle(25.0, 60.0, 20.0, 20.0, GREEN);
        draw_text("Score : 0", 20.0, 40.0, 30.0, BLACK);

        next_frame().await
    }
}
