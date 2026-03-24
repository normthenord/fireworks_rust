mod particle;
mod firework;
use macroquad::prelude::*;
use particle::*;
use firework::*;

#[macroquad::main("Fireworks!")]
async fn main() {
    let mut particle = Particle::new(100., screen_height(), 5.0)
        .with_speed(Vec2 { x: 0.0, y: -10.0 })
        .with_acceleration(Vec2 { x: 0.0, y: 0.1 });
    let mut firework = Firework::new(particle, RED);
    loop {
        clear_background(BLACK);

        firework.rocket.update();
        firework.rocket.draw();
        next_frame().await
    }
}
