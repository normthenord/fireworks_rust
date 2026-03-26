mod colors_list;
mod firework;
mod particle;
use firework::*;
use macroquad::{
    prelude::*,
    rand::{self, gen_range},
};
use particle::*;

use crate::colors_list::{random_color, random_macroquad_color};

#[macroquad::main("Fireworks!")]
async fn main() {
    set_fullscreen(true);
    let mut fireworks = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);

        if rand::gen_range(0, 10000) < 300 {
            make_firework(&mut fireworks);
        }

        for firework in &mut fireworks {
            firework.update();
            firework.draw();
            firework.age();
        }

        fireworks.retain(|f| !f.exploded || f.particles.iter().any(|p| p.alive));

        next_frame().await
    }
}

fn make_firework(fireworks: &mut Vec<Firework>) {
    let firework_radius = 5.0;

    let x_pos = rand::gen_range(firework_radius, screen_width() - firework_radius);
    let y_velocity = rand::gen_range(-5, -13);
    let color = random_macroquad_color();
    let particle = Particle::new(x_pos, screen_height(), firework_radius, color)
        .with_speed(Vec2 {
            x: 0.0,
            y: y_velocity as f32,
        })
        .with_acceleration(Vec2 { x: 0.0, y: 0.05 });

    let firework = Firework::new(particle, color);
    fireworks.push(firework);
}
