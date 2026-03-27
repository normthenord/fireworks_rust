mod colors_list;
mod firework;
mod particle;
use firework::*;
use macroquad::{
    prelude::*,
};

// use crate::colors_list::{random_color, random_macroquad_color};

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
            fireworks.push(Firework::new());
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

