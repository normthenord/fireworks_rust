mod particle;
use macroquad::prelude::*;
use particle::*;

#[macroquad::main("Fireworks!")]
async fn main() {
    loop {
        clear_background(BLACK);
        let mut particle = Particle::new(100., 100.)
            .with_speed(Vec2 { x: 2., y: 2. })
            .with_acceleration(Vec2 { x: 5., y: 4. });

        particle.update();
        particle.draw();
        next_frame().await
    }
}
