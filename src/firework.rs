use std::f32::consts::TAU;

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
        let gravity = Vec2 { x: 0.0, y: 0.05 };
        let firework_radius = 5.0;

        let x_pos = rand::gen_range(firework_radius, screen_width() - firework_radius);

        let min_height = screen_height() / 3.0;
        let max_height = screen_height() - firework_radius * 2.0;

        //v0 = sqrt(-2 * g * h)
        let min_velo = -(2.0 * gravity.y * min_height).sqrt();
        let max_velo = -(2.0 * gravity.y * max_height).sqrt();

        let y_velocity = rand::gen_range(min_velo, max_velo);
        let color = random_macroquad_color();
        let rocket = Particle::new(x_pos, screen_height(), firework_radius, color)
            .with_speed(Vec2 {
                x: 0.0,
                y: y_velocity as f32,
            })
            .with_acceleration(gravity);

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
            // self.rocket.draw();
            draw_rectangle(
                self.rocket.position.x,
                self.rocket.position.y,
                6.0,
                10.0,
                self.color,
            );
        }
        for particle in &self.particles {
            particle.draw();
        }
    }

    pub fn explode(&mut self) {
        self.exploded = true;
        let max_speed = 3.0;

        for _ in 0..50 {
            let angle = rand::gen_range(0.0, TAU);
            let radius = rand::gen_range(0.0, 1.0) * max_speed;
            // let radius = rand::gen_range(0.5f32, 1.0).powf(2.0) * max_speed;

            let x_speed = radius * angle.cos();
            let y_speed = radius * angle.sin();
            let particle = Particle::new(
                self.rocket.position.x,
                self.rocket.position.y,
                3.0,
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
