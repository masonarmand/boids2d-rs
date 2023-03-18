use macroquad::prelude::*;
use ::rand::prelude::*;
mod boid;
mod obstacle;
mod food;
use boid::*;
use obstacle::*;
use food::*;

const BOID_COUNT: i32 = 200;

fn window_conf() -> Conf {
    Conf {
        window_title: "boids 2d".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let mut boid_vec: Vec<Boid> = Vec::new();
    let mut obstacles_vec: Vec<Obstacle> = Vec::new();
    let mut food_vec: Vec<Food> = Vec::new();

    for _ in 0..BOID_COUNT {
        let x: i32 = rng.gen_range(80..screen_width() as i32 - 80);
        let y: i32 = rng.gen_range(80..screen_height() as i32 - 80);

        boid_vec.push(Boid {
            position: vec2(x as f32, y as f32),
            ..Default::default()
        });
    }

    loop {
        // Update
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos: Vec2 = vec2(mouse_position().0, mouse_position().1);
            let radius: i32 = rng.gen_range(10..30);

            obstacles_vec.push(Obstacle {
                position: pos,
                radius: radius as f32,
                ..Default::default()
            });
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let pos: Vec2 = vec2(mouse_position().0, mouse_position().1);

            food_vec.push(Food {
                position: pos,
                ..Default::default()
            });
        }

        // Render
        clear_background(WHITE);

        let boids_clone = boid_vec.clone();

        for boid in boid_vec.iter_mut() {
            boid.update(&boids_clone, &obstacles_vec, &mut food_vec);
            boid.draw();
        }

        for obstacle in obstacles_vec.iter_mut() {
            obstacle.draw();
        }

        for food in food_vec.iter_mut() {
            food.draw();
        }
        let fps_str: &str = &get_fps().to_string()[..];
        draw_text(fps_str, screen_width() - 60.0, screen_height() - 22.0, 22.0, BLACK);
        next_frame().await;
    }
}
