mod colors_list;
mod firework;
mod particle;
use anyhow::Result;
use firework::*;
use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams, Sound},
    prelude::*,
};

// const PATH: &str = "src/assets/1812overture.ogg";

#[macroquad::main("Fireworks!")]
async fn main() -> Result<()> {
    set_fullscreen(true);
    // let _maybe_sound = maybe_sound(PATH).await;
    let firework_sound = load_sound("assets/firework_sound.ogg").await?;
    let firework_loud = load_sound("assets/firework_loud.ogg").await?;
    let fizzle_sound = load_sound("assets/firework_fizzle.ogg").await.unwrap_or_else(|_| firework_sound.clone());
    let mut fireworks = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Q) {
            return Ok(());
        }

        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mut firework = Firework::new();
            firework.rocket.position = mouse_position().into();
            firework.rocket.velocity = Vec2::default();
            fireworks.push(firework);
        }

        if rand::gen_range(0, 10000) < 100 {
            fireworks.push(Firework::new());
        }

        for firework in &mut fireworks {
            if firework.update() {
                match firework.firework_type {
                    FireworkType::BOOM => {
                        play_sound(
                            &firework_loud,
                            PlaySoundParams {
                                volume: 1.0,
                                ..Default::default()
                            },
                        );
                    }
                    _ => {
                        play_sound(
                            &firework_sound,
                            PlaySoundParams {
                                volume: 1.0,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
            if firework.secondary_explosion_count > 0 {
                play_sound(
                    &fizzle_sound,
                    PlaySoundParams {
                        volume: 0.2, // maybe quieter for fizzle
                        ..Default::default()
                    },
                );
            }
            firework.draw();
            firework.age();
        }
        draw_frame_rate();
        fireworks.retain(|f| !f.exploded || f.particles.iter().any(|p| p.alive));
        next_frame().await
    }
}

fn draw_frame_rate() {
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
