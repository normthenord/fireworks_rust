mod colors_list;
mod firework;
mod particle;
use firework::*;
use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams, Sound},
    prelude::*,
};

// const PATH: &str = "src/assets/1812overture.ogg";

#[macroquad::main("Fireworks!")]
async fn main() {
    set_fullscreen(true);
    // clear_background(BLACK);
    // let _maybe_sound = maybe_sound(PATH).await;
    let firework_sound = load_sound("assets/firework_sound.ogg").await.unwrap();
    let mut fireworks = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        // let color = Color::new(0.0, 0.0, 0.0, 0.0001);
        // draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color);

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
            if firework.update() {
                play_sound(
                    &firework_sound,
                    PlaySoundParams {
                        volume: 1.0,
                        ..Default::default()
                    },
                );
            }
            firework.draw();
            firework.age();
        }
        frame_rate();
        fireworks.retain(|f| !f.exploded || f.particles.iter().any(|p| p.alive));
        next_frame().await
    }
}

fn frame_rate() {
    let frame_time = get_frame_time();
    let fps = 1.0 / frame_time;

    let fps_text = format!("FPS: {}", fps.round());
    draw_text(&fps_text, 10.0, 20.0, 20.0, WHITE);
}

#[allow(dead_code)]
async fn maybe_sound(path: &str) -> Option<Sound> {
    match load_sound(path).await {
        Ok(sound) => {
            play_sound(
                &sound,
                PlaySoundParams {
                    looped: true,
                    volume: 1.0,
                },
            );
            Some(sound)
        }
        Err(_) => None,
    }
}
