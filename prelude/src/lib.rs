pub use raylib_ffi::*;


pub struct RlWindow;
impl RlWindow {
    /// Initialize Raylib window
    pub unsafe fn init_window(width: i32, height: i32, title: *const i8) -> Self{
        InitWindow(width, height, title);
        RlWindow
    }
    /// Checks if Raylib window should close
    pub unsafe fn window_should_close(&self) -> bool {
        WindowShouldClose()
    }
    /// Handles Raylib drawing contex
    pub unsafe fn draw_mode<F>(&self, f: F)
    where F: FnOnce()
    {
        BeginDrawing();
        f();
        EndDrawing();
    }
    /// Set Raylib target FPS
    pub unsafe fn set_target_fps(&self, fps: i32 ) {
        SetTargetFPS(fps);
    }
}

impl Drop for RlWindow {
    fn drop(&mut self) {
        unsafe {
            CloseWindow();
        }
    }
}

pub fn color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
    unsafe { ColorFromHSV(hue, saturation, value) }
}

pub fn color_lerp(a: Color, b: Color, t: f64) -> Color {
    Color {
        r: (a.r as f64 * (1.0 - t) + b.r as f64 * t) as u8,
        g: (a.g as f64 * (1.0 - t) + b.g as f64 * t) as u8,
        b: (a.b as f64 * (1.0 - t) + b.b as f64 * t) as u8,
        a: (a.a as f64 * (1.0 - t) + b.a as f64 * t) as u8,
    }
}
