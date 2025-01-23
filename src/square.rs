use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Square {
    position: Vec2,
    color: Color,
}

impl Square {
    fn new(position: Vec2, color: Color) -> Self {
        Square {
            position,
            color,
        }
    }

    fn replace(&mut self, other: Square) -> Self {
        std::mem::replace(self, other)
    }
}

pub async fn main() {
    let mut rotation: f32 = 90.;
    let mut squares: Vec<Square> = (0..1000).map(|_| {
        Square::new(
            Vec2::new(screen_width() / 2., screen_height() / 2.),
            RED,
        )
    }).collect();

    loop {
        if is_key_down(KeyCode::A) {
            rotation += 2.;
        }
        if is_key_down(KeyCode::D) {
            rotation -= 2.;
        }

        let new_x = 5. * rotation.to_radians().cos() / (get_fps() as f32 / 45.);
        let new_y = 5. * -rotation.to_radians().sin() / (get_fps() as f32 / 45.);

        let mut last = squares.remove(0);
        last.position.x += new_x;
        last.position.y += new_y;
        squares.insert(0, last);

        for square in squares.iter_mut() {
            if (square.position.x - last.position.x).abs() > 20. || (square.position.y - last.position.y).abs() > 20.{
                last = square.replace(last);
            }
            draw_rectangle(
                square.position.x, 
                square.position.y, 
                5., 
                5., 
                square.color
            );
        }
        next_frame().await;
    }
}
