use std::f32::consts::TAU;

use crate::colors_list::random_macroquad_color;
use crate::particle::*;
use macroquad::color::Color;
use macroquad::prelude::*;
// use macroquad::rand::gen_range;

use ::rand::distr::weighted::WeightedIndex;
use ::rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Firework {
    pub rocket: Particle,
    pub particles: Vec<Particle>,
    pub color: Color,
    pub exploded: bool,
    pub firework_type: FireworkType,
    pub secondary_explosion_count: u32,
}

#[derive(Debug, EnumIter, Clone)]
pub enum FireworkType {
    RING,
    NORMAL,
    WILLOW,
    CASCADE,
    BOOM,
}

impl FireworkType {
    pub fn random() -> FireworkType {
        let choices: Vec<FireworkType> = FireworkType::iter().collect();
        let weights = choices.iter().map(|c| c.weight()).collect::<Vec<u32>>();

        let dist = WeightedIndex::new(&weights).unwrap();

        let mut rand = ::rand::rng();
        let selected = &choices[dist.sample(&mut rand)];

        selected.clone()
    }

    fn weight(&self) -> u32 {
        match self {
            FireworkType::RING => 20,
            FireworkType::NORMAL => 30,
            FireworkType::WILLOW => 20,
            FireworkType::CASCADE => 20,
            FireworkType::BOOM => 5,
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
            secondary_explosion_count: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        self.secondary_explosion_count = 0;
        self.rocket.update();
        if self.rocket.velocity.y >= 0.0 && !self.exploded {
            self.explode();
            return true;
        }

        for particle in &mut self.particles {
            particle.update();
        }

        // Handle secondary explosions
        let mut explosions = vec![];
        for particle in &self.particles {
            if particle.explosive && particle.explosion_timer <= 0.0 && particle.alive {
                explosions.push((particle.position, particle.color));
            }
        }
        self.secondary_explosion_count = explosions.len() as u32;
        for (pos, color) in explosions {
            for _ in 0..15 {
                let angle = rand::gen_range(0.0, TAU);
                let speed = 2.0;
                let x_speed = speed * angle.cos();
                let y_speed = speed * angle.sin();
                let new_particle = Particle::new(pos.x, pos.y, 2.0, color)
                    .with_speed(Vec2 {
                        x: x_speed,
                        y: y_speed,
                    })
                    .with_acceleration(Vec2 { x: 0.0, y: 0.01 })
                    .with_dampening(Vec2 { x: 0.98, y: 0.98 });
                self.particles.push(new_particle);
            }
        }
        // Mark exploded particles as non-explosive
        for particle in &mut self.particles {
            if particle.explosive && particle.explosion_timer <= 0.0 {
                particle.explosive = false;
            }
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
            FireworkType::CASCADE => self.explode_cascade(),
            FireworkType::BOOM => self.explode_boom(),
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

    fn explode_cascade(&mut self) {
        let max_speed = 3.0;
        for _ in 0..20 {
            let radius = rand::gen_range(3.0, 5.0);
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
            .with_acceleration(Vec2 { x: 0.0, y: 0.01 })
            .with_dampening(Vec2 { x: 0.98, y: 0.98 })
            .with_explosive(1.0); // explode after 1.0 seconds (100 frames at 0.01 decrement)
            self.particles.push(particle);
        }
    }

    fn explode_boom(&mut self) {
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

    pub fn age(&mut self) {
        for particle in &mut self.particles {
            match self.firework_type {
                FireworkType::RING => particle.lifetime -= 0.005,
                FireworkType::NORMAL => particle.lifetime -= 0.005,
                FireworkType::WILLOW => particle.lifetime -= 0.0025,
                FireworkType::CASCADE => particle.lifetime -= 0.005,
                FireworkType::BOOM => particle.lifetime -= 0.03,
            }
        }
    }
}
