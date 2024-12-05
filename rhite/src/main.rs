use mar::MapBuilder;
use prelude::*;

const BACKGROUND_COLOR_HEX: u32 = 0x181818FF;
const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 20;

fn main() -> Result<(), ()> {
    unsafe {
        let map = MapBuilder::new(WIDTH, HEIGHT, CELL_SIZE)
            .map_err(|e| eprintln!("ERROR: {e}"))?
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
        let w = RlWindow;

        w.init_window(WIDTH as i32, HEIGHT as i32, rl_str!("rhite"));
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(bg_color);
            map.draw();
            EndDrawing();
        }
    };

    Ok(())
}

struct RlWindow;
impl RlWindow {
    pub unsafe fn init_window(&self, width: i32, height: i32, title: *const i8) {
        InitWindow(width, height, title);
    }
}

impl Drop for RlWindow {
    fn drop(&mut self) {
        unsafe {
            CloseWindow();
        }
    }
}
