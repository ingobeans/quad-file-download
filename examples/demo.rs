use macroquad::prelude::*;
use quad_file_download::*;
#[macroquad::main("example :>")]
async fn main() {
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Space) {
        	download("wa.txt",b"hello").unwrap();
        }

        draw_text("press [space] to save a cool file!", 20.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
