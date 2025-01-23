use macroquad::prelude::*;

pub async fn main() {
    let hour_size = 32.;
    let x = screen_width() / 2.;
    let y = screen_height() / 2.;
    let radius = screen_width().min(screen_height()) / 2.;
    loop {
        draw_circle(x, y, radius, WHITE);
        draw_circle(x, y, radius * 0.9, BLACK);

        for hour in 0..12 {
            let degree = (hour as f32 / 12.) * 360. - 60.0;
            let hour = hour + 1;
            draw_text(
                &format!("{hour}"),
                x + (radius * degree.to_radians().cos() * 0.8) - (hour_size / 4.),
                y + (radius * degree.to_radians().sin() * 0.8) + (hour_size / 4.),
                hour_size,
                RED,
            );
        }

        for dot in 0..(5 * 12) {
            let degree = dot as f32 / (5.0 * 12.0) * 360.0;
            let size = if dot % 5 == 0 { 3.0 } else { 1.0 };
            draw_circle(
                x + (radius * degree.to_radians().cos() * 0.7),
                y + (radius * degree.to_radians().sin() * 0.7),
                size,
                BLUE,
            );
        }

        next_frame().await
    }
}
