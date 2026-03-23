use macroquad::{color::{Color, RED}, shapes::draw_circle};
use macroquad::prelude::*;
use vec2::transform_mat4_projection_mut;



#[derive(Debug)]
pub struct Particle {
    position: Vec2,
    velocity:  Vec2,
    acceleration: Vec2,
    lifetime: f64,
    color: Color
}


impl Particle{
   pub fn new(x: f32, y: f32) -> Particle{
        Particle { position: Vec2 { x, y }, velocity: Vec2::default(), acceleration: Vec2::default(), lifetime: 1.0, color: Color::default()}
    }


    pub fn with_speed(self, velocity: Vec2) -> Particle{
        Particle{velocity, ..self}
    }

    pub fn with_acceleration(self, acceleration: Vec2) -> Particle { 
        Particle {acceleration, ..self}
    }


    pub fn update(&mut self) {

        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vec2 { x: 0.0, y: 1.0 };
        

    }
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 3.0, RED);
        println!("{}", self.position);
    }


}