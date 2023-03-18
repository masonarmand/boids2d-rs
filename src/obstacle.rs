use macroquad::prelude::*;

#[derive(Clone)]
pub struct Obstacle {
    pub position: Vec2,
    pub radius: f32,
    pub color: Color
}

impl Obstacle {
    pub fn draw(&mut self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}

impl Default for Obstacle {
    fn default() -> Obstacle {
        Obstacle {
            position: Vec2::ZERO,
            radius: 10.0,
            color: BLUE
        }
    }
}
