mod firework;
mod particle;
use firework::*;
use macroquad::{prelude::*, rand};
use particle::*;

#[macroquad::main("Fireworks!")]
async fn main() {
    let mut fireworks = Vec::new();

    loop {
        clear_background(BLACK);

        if rand::gen_range(0, 10000) < 300 {
            make_firework(&mut fireworks);
        }

        for firework in &mut fireworks {
            firework.update();
            firework.draw();
        }

        fireworks.retain(|f| !f.exploded || f.particles.iter().any(|p| p.alive));

        next_frame().await
    }
}

fn make_firework(fireworks: &mut Vec<Firework>) {
    let firework_radius = 5.0;

    let x_pos = rand::gen_range(firework_radius, screen_width() - firework_radius);
    let y_velocity = rand::gen_range(-8, -12);
    let particle = Particle::new(x_pos, screen_height(), firework_radius)
        .with_speed(Vec2 {
            x: 0.0,
            y: y_velocity as f32,
        })
        .with_acceleration(Vec2 { x: 0.0, y: 0.1 });

    let firework = Firework::new(particle, WHITE);
    fireworks.push(firework);
}
