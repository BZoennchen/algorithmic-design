use nannou::prelude::*;

#[derive(Default)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    max_velocity: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Particle { position, velocity, acceleration, max_velocity: f32::MAX, }
    }

    pub fn from_position(position: Vec2, max_velocty: f32) -> Self {
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let mut particle = Self::new(position, velocity, acceleration);
        particle.max_velocity = max_velocty;
        return particle;
    }

    fn accelerate(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.velocity = self.velocity.clamp_length(0.0, self.max_velocity);
    }

    fn movement(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn update(&mut self, dt: f32) {
        self.accelerate(dt);
        self.movement(dt);
    }
}