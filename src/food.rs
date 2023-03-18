use macroquad::prelude::*;

#[derive(Clone)]
pub struct Food {
    pub position: Vec2,
    pub radius: f32,
    pub color: Color
}

impl Food {
    pub fn draw(&mut self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}

impl Default for Food {
    fn default() -> Food {
        Food {
            position: Vec2::ZERO,
            radius: 3.0,
            color: BROWN
        }
    }
}
