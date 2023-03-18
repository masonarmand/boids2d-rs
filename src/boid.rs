use macroquad::prelude::*;
use crate::obstacle::*;
use crate::food::*;

#[derive(Clone)]
pub struct Boid {
    pub visual_range: f32,
    pub repulsion_range: f32,

    pub turn_factor: f32,
    pub centering_factor: f32,
    pub avoid_factor: f32,
    pub matching_factor: f32,

    pub max_speed: f32,
    pub min_speed: f32,

    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,

    pub color: Color,
}

impl Boid {
    pub fn update(&mut self, boids: &Vec<Boid>, obstacles: &Vec<Obstacle>, foods: &mut Vec<Food>) {
        let mut pos_avg: Vec2 = Vec2::ZERO;
        let mut vel_avg: Vec2 = Vec2::ZERO;
        let mut neighbors: i32 = 0;
        let mut repulsion_pos: Vec2 = Vec2::ZERO;

        for boid in boids {
            let diff: Vec2 = self.position - boid.position;

            if diff.length() < self.visual_range {
                if diff.length() < self.repulsion_range {
                    repulsion_pos += diff;
                }
                pos_avg += boid.position;
                vel_avg += boid.velocity;

                neighbors += 1;
            }
        }

        // Obstacle collision/avoidance
        for obstacle in obstacles {
            let diff: Vec2 = self.position - obstacle.position;

            if diff.length() < obstacle.radius {
                repulsion_pos += diff;
            }
        }

        // Goto position of food
        if foods.len() > 0 {
            let diff: Vec2 = self.position - foods[0].position;
            pos_avg -= diff;
            if diff.length() < foods[0].radius {
                foods.remove(0);
            }
        }

        if neighbors > 0 {
            pos_avg = vec2(pos_avg.x / neighbors as f32, pos_avg.y / neighbors as f32);
            vel_avg = vec2(vel_avg.x / neighbors as f32, vel_avg.y / neighbors as f32);

            self.velocity += (pos_avg - self.position) * self.centering_factor +
                             (vel_avg - self.velocity) * self.matching_factor;

            self.velocity += repulsion_pos * self.avoid_factor;
        }

        // Speed limit
        let speed: f32 = f32::sqrt(self.velocity.x * self.velocity.x +
                                   self.velocity.y * self.velocity.y);

        if speed < self.min_speed {
            self.velocity = (self.velocity/speed) * self.min_speed
        }
        else if speed > self.max_speed {
            self.velocity = (self.velocity/speed) * self.max_speed
        }

        // Boundries
        let margin: f32 = 50.0;

        if self.position.x < margin {
            self.velocity.x += self.turn_factor;
        }
        if self.position.x > 640.0 - margin {
            self.velocity.x -= self.turn_factor;
        }
        if self.position.y < margin {
            self.velocity.y += self.turn_factor;
        }
        if self.position.y > 480.0 - margin {
            self.velocity.y -= self.turn_factor;
        }

        self.position += self.velocity;
    }

    pub fn draw(&mut self) {
        self.rotation = f32::to_degrees(f32::atan2(self.velocity.y, self.velocity.x));
        draw_poly(self.position.x, self.position.y, 3, 4.0, self.rotation, self.color);
    }
}

impl Default for Boid {
    fn default() -> Boid {
        Boid {
            turn_factor: 0.2,
            visual_range: 20.0,
            repulsion_range: 10.0,
            centering_factor: 0.005,
            avoid_factor: 0.05,
            matching_factor: 0.05,
            max_speed: 2.0,
            min_speed: 1.0,
            position: vec2(10.0, 10.0),
            velocity: vec2(1.0, 1.0),
            rotation: 0.0,
            color: BLACK,
        }
    }
}
