mod colors_list;
mod firework;
mod particle;
use firework::*;
use macroquad::{
    prelude::*,
};


#[macroquad::main("Fireworks!")]
async fn main() {
    set_fullscreen(true);
    let mut fireworks = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);


        if is_mouse_button_pressed(MouseButton::Left) {
            let mut firework = Firework::new();
            firework.rocket.position = mouse_position().into();
            firework.rocket.velocity = Vec2::default();
            fireworks.push(firework);
        }

        if rand::gen_range(0, 10000) < 500 {
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

