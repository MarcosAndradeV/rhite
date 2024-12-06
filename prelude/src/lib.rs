use std::ops;

use enums::KeyboardKey;
pub use raylib_ffi::*;

pub struct RlWindow;
impl RlWindow {
    /// Initialize Raylib window
    pub fn init_window(width: i32, height: i32, title: *const i8) -> Self {
        unsafe { InitWindow(width, height, title); }
        RlWindow
    }
    /// Checks if Raylib window should close
    pub fn window_should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }
    /// Handles Raylib drawing context
    pub fn draw_mode<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        unsafe {
            BeginDrawing();
            f();
            EndDrawing();
        }
    }
    /// Handles Raylib camera2D context
    pub fn camera2d_mode<F>(&self, camera: RlCamera2D, f: F)
    where
        F: Fn(),
    {
        unsafe {
            BeginDrawing();
            BeginMode2D(camera.into());
            f();
            EndMode2D();
            EndDrawing();
        }
    }
    /// Set Raylib target FPS
    pub fn set_target_fps(&self, fps: i32) {
        unsafe { SetTargetFPS(fps); }
    }
}

impl Drop for RlWindow {
    fn drop(&mut self) {
        unsafe {
            CloseWindow();
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlCamera2D {
    pub offset: RlVector2,
    pub target: RlVector2,
    pub rotation: f32,
    pub zoom: f32,
}
impl RlCamera2D {
    pub fn new(offset: RlVector2, target: RlVector2, rotation: f32, zoom: f32) -> RlCamera2D {
        Self {
            offset,
            target,
            rotation,
            zoom,
        }
    }
    pub fn zero() -> RlCamera2D {
        Self::new(RlVector2::zero(), RlVector2::zero(), 0.0, 1.0)
    }
    pub fn into(self) -> Camera2D {
        Camera2D {
            offset: self.offset.into(),
            target: self.target.into(),
            rotation: self.rotation,
            zoom: self.zoom,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlVector2 {
    pub x: f32,
    pub y: f32,
}
impl RlVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn into(self) -> Vector2 {
        Vector2{x:self.x,y:self.y}
    }
    pub fn scale(self, s: f32) -> Self {
        Self { x: self.x*s, y: self.y*s }
    }
    pub fn scale_mut(&mut self, s: f32) {
        self.x*=s; self.y*=s;
    }
}
impl ops::Add for RlVector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x+rhs.x, y: self.y+rhs.y }
    }
}
impl From<Vector2> for RlVector2 {
    fn from(value: Vector2) -> Self {
        Self { x:value.x,y:value.y }
    }
}

impl Default for RlVector2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlRectangle{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl RlRectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn into(self) -> Rectangle {
        Rectangle { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}
impl From<Rectangle> for RlRectangle {
    fn from(value: Rectangle) -> Self {
        Self { x: value.x, y: value.y, width: value.width, height: value.height }
    }
}

impl Default for RlRectangle {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl RlColor {
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        unsafe { ColorFromHSV(hue, saturation, value) }
    }
    pub fn lerp(a: Color, b: Color, t: f64) -> Color {
        Color {
            r: (a.r as f64 * (1.0 - t) + b.r as f64 * t) as u8,
            g: (a.g as f64 * (1.0 - t) + b.g as f64 * t) as u8,
            b: (a.b as f64 * (1.0 - t) + b.b as f64 * t) as u8,
            a: (a.a as f64 * (1.0 - t) + b.a as f64 * t) as u8,
        }
    }
    pub fn get_color(hex: u32) -> Color {
        unsafe { GetColor(hex) }
    }
    pub fn into(self) -> Color {
        Color { r: self.r, g: self.g, b: self.b, a:  self.a }
    }
}

pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe {
        IsKeyDown(key as i32)
    }
}
