use macroquad::prelude::*;

struct Triangle {
    x: f32,
    y: f32,
    rot: f32
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut triangle = Triangle {
        x: screen_width() / 2., y: screen_height() / 2., rot: 0.
    };
    loop {
        if is_key_down(KeyCode::A) {
            triangle.rot += 2.;
        }
        if is_key_down(KeyCode::D) {
            triangle.rot -= 2.;
        }
        if is_key_down(KeyCode::W) {
            triangle.x -= triangle.rot.to_radians().sin();
            triangle.y -= triangle.rot.to_radians().cos();
        }
        draw_rectangle(triangle.x, triangle.y, 10., 10., RED);
        next_frame().await
    }
}