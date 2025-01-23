mod clock;
mod square;

#[macroquad::main("MyGame")]
async fn main() {
    clock::main().await;
}
