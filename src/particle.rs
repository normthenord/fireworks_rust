use macroquad::prelude::*;
use macroquad::{
    color::{Color, RED},
    shapes::draw_circle,
};

#[derive(Debug)]
pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    radius: f32,
    lifetime: f64,
    alive: bool,
    color: Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, radius: f32) -> Particle {
        Particle {
            position: Vec2 { x, y },
            velocity: Vec2::default(),
            acceleration: Vec2::default(),
            radius,
            lifetime: 1.0,
            alive: true,
            color: RED,
        }
    }

    pub fn with_speed(self, velocity: Vec2) -> Particle {
        Particle { velocity, ..self }
    }

    pub fn with_acceleration(self, acceleration: Vec2) -> Particle {
        Particle {
            acceleration,
            ..self
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        // self.acceleration = Vec2 { x: 0.0, y: 0.2 };
        if self.lifetime < 0.0 {
            self.alive = false;
        }
    }
    pub fn draw(&self) {
        if self.alive {
            draw_circle(self.position.x, self.position.y, self.radius, self.color);
            println!("{}", self.position);
        }
    }
}
