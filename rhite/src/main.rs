use mar::{Entity, Player};
use prelude::*;

const BACKGROUND_COLOR_HEX: u32 = 0x181818FF;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> Result<(), ()> {
    let w = RlWindow::init_window(WIDTH as i32, HEIGHT as i32, rl_str!("rhite"));
    w.set_target_fps(60);

    let mut camera = RlCamera2D::zero();
    camera.zoom = 1.0;
    camera.offset = RlVector2::new((WIDTH/2) as f32, (HEIGHT/2) as f32);

    let mut p = Player::new(RlRectangle::new(400.0, 225.0, 50.0, 50.0), 5.0);

    let bg_color = RlColor::get_color(BACKGROUND_COLOR_HEX);
    while !w.window_should_close() {
        unsafe {
            p.update();
            camera.target = p.get_pos() + RlVector2::new(20.0, 20.0);

            w.camera2d_mode(camera, || {
                ClearBackground(bg_color);
                p.draw();
                DrawFPS(10, 10);
            });
        }
    }

    Ok(())
}
