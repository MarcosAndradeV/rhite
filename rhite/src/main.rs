use mar::MapBuilder;
use prelude::*;
use rand::{thread_rng, Rng};

const BACKGROUND_COLOR_HEX: u32 = 0x181818FF;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BLOCK_SIZE: usize = 20;

fn main() -> Result<(), ()> {
    unsafe {
        let map = MapBuilder::new(WIDTH, HEIGHT, BLOCK_SIZE)
            .map_err(|e| eprintln!("ERROR: {e}"))?
            .perlin_noise(thread_rng().gen())
            .set_many(&[
                (1, 1, colors::RED),
                (2, 2, colors::BLUE),
                (3, 3, colors::GREEN),
            ])
            .fill_vertical(0, colors::GRAY)
            .fill_horizontal(0, colors::GRAY)
            .apply(|mb| mb.fill_horizontal(20, colors::RED))
            .fill_horizontal_strip(2, 5, 17, colors::GOLD)
            .fill_vertical_strip(2, 5, 17, colors::SKYBLUE)
            .build();

        let bg_color = GetColor(BACKGROUND_COLOR_HEX);
        let w = RlWindow::init_window(WIDTH as i32, HEIGHT as i32, rl_str!("rhite"));
        w.set_target_fps(60);
        while !w.window_should_close() {
            w.draw_mode(|| {
                ClearBackground(bg_color);
                map.draw();
            });
        }
    };

    Ok(())
}
