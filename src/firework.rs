use std::f32::consts::TAU;

use crate::colors_list::random_macroquad_color;
use crate::particle::*;
use macroquad::color::Color;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct Firework {
    pub rocket: Particle,
    pub particles: Vec<Particle>,
    pub color: Color,
    pub exploded: bool,
    pub firework_type: FireworkType,
}

pub enum FireworkType {
    RING,
    NORMAL,
    WILLOW,
}

impl FireworkType {
    pub fn random() -> FireworkType {
        match gen_range(0, 100) {
            0..20 => FireworkType::RING,
            20..50 => FireworkType::NORMAL,
            50..70 => FireworkType::WILLOW,
            _ => FireworkType::NORMAL,
        }
    }
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

        let firework_type = FireworkType::random();

        Firework {
            rocket,
            particles: Vec::new(),
            color,
            exploded: false,
            firework_type: firework_type,
        }
    }

    pub fn update(&mut self) -> bool {
        self.rocket.update();
        if self.rocket.velocity.y >= 0.0 && !self.exploded {
            self.explode();
            return true;
        }

        for particle in &mut self.particles {
            particle.update();
        }
        return false;
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
        match self.firework_type {
            FireworkType::RING => self.explode_ring(),
            FireworkType::NORMAL => self.explode_normal(),
            FireworkType::WILLOW => self.explode_willow(),
        }
    }

    fn explode_ring(&mut self) {
        let speed = 3.0;

        for _ in 0..150 {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let radius = rand::gen_range(2.0, 4.0);
            // Tight speed range = clean spherical shell
            let explosion_radius = rand::gen_range(0.8, 1.0) * speed;

            let x_speed = explosion_radius * angle.cos();
            let y_speed = explosion_radius * angle.sin();

            let particle = Particle::new(
                self.rocket.position.x,
                self.rocket.position.y,
                radius, // slightly smaller particles look better
                self.color,
            )
            .with_speed(Vec2 {
                x: x_speed,
                y: y_speed,
            })
            .with_acceleration(Vec2 { x: 0.0, y: 0.01 }) // gravity
            .with_dampening(Vec2 { x: 0.995, y: 0.995 }); // very low drag

            self.particles.push(particle);
        }
    }

    fn explode_normal(&mut self) {
        let max_speed = 3.0;

        for _ in 0..50 {
            let angle = rand::gen_range(0.0, TAU);
            let explosion_radius = rand::gen_range(0.0, 1.0) * max_speed;
            let radius = rand::gen_range(2.0, 4.0);

            let x_speed = explosion_radius * angle.cos();
            let y_speed = explosion_radius * angle.sin();
            let particle = Particle::new(
                self.rocket.position.x,
                self.rocket.position.y,
                radius,
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

    fn explode_willow(&mut self) {
        let max_speed = 3.0;
        for _ in 0..30 {
            let radius = rand::gen_range(2.0, 4.0);
            let angle = rand::gen_range(0.0, TAU);
            let explosion_radius = rand::gen_range(0.0, 1.0) * max_speed;

            let x_speed = explosion_radius * angle.cos();
            let y_speed = explosion_radius * angle.sin();
        
            let particle = Particle::new(
                self.rocket.position.x,
                self.rocket.position.y,
                radius,
                self.color,
            )
            .with_speed(Vec2 {
                x: x_speed,
                y: y_speed,
            })
            .with_acceleration(Vec2 { x: 0.0, y: 0.02 })
            .with_dampening(Vec2 { x: 0.99, y: 0.99 });
            self.particles.push(particle);
        }
    }

    pub fn age(&mut self) {
        for particle in &mut self.particles {
            match self.firework_type {
                FireworkType::RING => particle.lifetime -= 0.005,
                FireworkType::NORMAL => particle.lifetime -= 0.005,
                FireworkType::WILLOW => particle.lifetime -= 0.0025,
            }
        }
    }
}
