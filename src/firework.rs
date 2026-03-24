use crate::particle::*;
use macroquad::color::Color;

pub struct Firework {
    pub rocket: Particle,
    pub particles: Vec<Particle>,
    pub color: Color,
}

impl Firework {
    pub fn new(rocket: Particle, color: Color) -> Firework {
        Firework {
            rocket,
            particles: Vec::new(),
            color,
        }
    }
}
