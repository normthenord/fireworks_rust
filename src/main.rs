mod particle;
use macroquad::prelude::*;
use particle::*;

#[macroquad::main("Fireworks!")]
async fn main() {
    let mut particle = Particle::new(100., 100., 20.0)
        .with_speed(Vec2 { x: 0.2, y: 0.20 })
        .with_acceleration(Vec2 { x: 0.0, y: 0.0 });
    loop {
        clear_background(BLACK);

        particle.update();
        particle.draw();
        next_frame().await
    }
}
