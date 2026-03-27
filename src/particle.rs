use macroquad::prelude::*;
use macroquad::{color::Color, shapes::draw_circle};

#[derive(Debug)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub lifetime: f32,
    pub alive: bool,
    pub color: Color,
    pub dampening: Vec2
}

impl Particle {
    pub fn new(x: f32, y: f32, radius: f32, color: Color) -> Particle {
        Particle {
            position: Vec2 { x, y },
            velocity: Vec2::default(),
            acceleration: Vec2::default(),
            radius,
            lifetime: 1.0,
            alive: true,
            color,
            dampening: Vec2{x: 1.0, y: 1.0}
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

    pub fn with_dampening (self, dampening: Vec2) -> Particle {
        Particle {dampening, ..self}
    }



    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity *= self.dampening;
        self.position += self.velocity;
        // self.acceleration = Vec2 { x: 0.0, y: 0.2 };
        self.color.a = self.lifetime as f32;
        if self.lifetime < 0.0 {
            self.alive = false;
        }
    }
    pub fn draw(&self) {
        if self.alive {
            draw_circle(self.position.x, self.position.y, self.radius, self.color);
        }
    }
}
