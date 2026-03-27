use crate::colors_list::random_macroquad_color;
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
    pub fn new() -> Firework {
        let firework_radius = 5.0;

        let x_pos = rand::gen_range(firework_radius, screen_width() - firework_radius);
        let y_velocity = rand::gen_range(-5, -13);
        let color = random_macroquad_color();
        let rocket = Particle::new(x_pos, screen_height(), firework_radius, color)
            .with_speed(Vec2 {
                x: 0.0,
                y: y_velocity as f32,
            })
            .with_acceleration(Vec2 { x: 0.0, y: 0.05 });

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

        for _ in 0..30 {
            let x_speed = rand::gen_range(-2.0, 2.0);
            let y_speed = rand::gen_range(-2.0, 2.0);
            let particle = Particle::new(
                self.rocket.position.x,
                self.rocket.position.y,
                2.0,
                self.color,
            )
            .with_speed(Vec2 {
                x: x_speed,
                y: y_speed,
            })
            .with_acceleration(Vec2 { x: 0.0, y: 0.01 })
            .with_dampening(Vec2 { x: 0.98, y: 0.98 });
            self.particles.push(particle);
        }
    }

    pub fn age(&mut self) {
        for particle in &mut self.particles {
            particle.lifetime -= 0.005;
        }
    }
}
