use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut x = 10.0;
    let mut y = 10.0;
    loop {
        if is_key_down(KeyCode::A) {
            x = x - 1.0;
        }
        if is_key_down(KeyCode::D) {
            x = x + 1.0;
        } 
        if is_key_down(KeyCode::S) {
            y = y + 1.0;
        }
        if is_key_down(KeyCode::W) {
            y = y - 1.0;
        }
        draw_rectangle(x, y, 1.0, 1.0, RED);
        next_frame().await
    }
}