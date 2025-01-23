mod clock;
mod square;

#[macroquad::main("MyGame")]
async fn main() {
    square::main().await;
}
