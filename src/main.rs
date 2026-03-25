mod firework;
mod particle;
use firework::*;
use macroquad::prelude::*;
use particle::*;

#[macroquad::main("Fireworks!")]
async fn main() {
    let mut fireworks = Vec::new();

    let particle = Particle::new(100., screen_height(), 5.0)
        .with_speed(Vec2 { x: 0.0, y: -10.0 })
        .with_acceleration(Vec2 { x: 0.0, y: 0.1 });

    let firework = Firework::new(particle, RED);
    fireworks.push(firework);

    loop {
        clear_background(BLACK);
        for firework in &mut fireworks {
            firework.update();
            firework.draw();
        }

        fireworks.retain(|f| !f.exploded || f.particles.iter().any(|p| p.alive));

        next_frame().await
    }
}
