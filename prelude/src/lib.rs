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
