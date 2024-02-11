use nannou::prelude::*;

const EPSILON: f32 = 0.01;

pub trait SignedDistanceFunction {
    fn distance(&self, point: Vec2) -> f32;
    fn grad(&self, point: Vec2) -> Vec2 {
        let hx: Vec2 = Vec2::new(EPSILON, 0.);
        let hy: Vec2 = Vec2::new(0., EPSILON);
        let dx = (self.distance(point+hx) - self.distance(point))/EPSILON;
        let dy = (self.distance(point+hy) - self.distance(point))/EPSILON;
        vec2(dx, dy)
    }
}

pub struct Rect {
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(center: Vec2, width: f32, height: f32) -> Self {
        Rect { center, width, height }
    }
}

impl SignedDistanceFunction for Rect {
    fn distance(&self, point: Vec2) -> f32 {
        let x_dist = f32::abs(self.center.x-point.x) - self.width;
        let y_dist = f32::abs(self.center.y-point.y) - self.height;
        f32::max(x_dist, y_dist)
    }
}

pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Circle { center, radius }
    }
}

impl SignedDistanceFunction for Circle {
    fn distance(&self, point: Vec2) -> f32 {
        self.center.distance(point) - self.radius
    }
}

pub struct SDFUnion {
    pub sdfs: Vec<Box<dyn SignedDistanceFunction>>,
}

impl SignedDistanceFunction for SDFUnion {
    fn distance(&self, point: Vec2) -> f32 {
        let mut dist = f32::MAX;
        
        for sdf in self.sdfs.iter() {
            dist = dist.min(sdf.distance(point));
        }

        dist
    }
}

impl SDFUnion {
    pub fn new(sdfs: Vec<Box<dyn SignedDistanceFunction>>) -> Self {
        SDFUnion { sdfs }
    }
}
