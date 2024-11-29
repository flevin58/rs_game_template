use raylib::prelude::*;

pub struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

impl Circle {
    pub fn new(x: u32, y: u32, r: u32) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
            r: r as i32,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x, self.y, self.r as f32, Color::GREEN);
    }
}
