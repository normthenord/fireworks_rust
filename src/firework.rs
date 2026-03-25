use crate::particle::*;
use macroquad::color::Color;
use macroquad::prelude::*;

pub struct Firework {
    pub rocket: Particle,
    pub particles: Vec<Particle>,
    pub color: Color,
    pub exploded: bool,
}

impl Firework {
    pub fn new(rocket: Particle, color: Color) -> Firework {
        Firework {
            rocket,
            particles: Vec::new(),
            color,
            exploded: false,
        }
    }

    pub fn update(&mut self) {
        self.rocket.update();
        if self.rocket.velocity.y >= 0.0 && !self.exploded {
            println!("Rocket exploded!");
            self.explode();
        }

        for particle in &mut self.particles {
            particle.update();
        }
    }

    pub fn draw(&self) {
        if !self.exploded {
            self.rocket.draw();
        }
        for particle in &self.particles {
            particle.draw();
        }
    }

    pub fn explode(&mut self) {
        self.exploded = true;

        for _ in 0..10 {
            let particle = Particle::new(self.rocket.position.x, self.rocket.position.y, 2.0)
                .with_speed(1.0 * Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)))
                .with_acceleration(Vec2 { x: 0.0, y: 0.1 });
            self.particles.push(particle);
        }
    }
}
