use nannou::prelude::*;

pub fn random_vec(w: f32, h: f32) -> Vec2 {
    vec2((random_f32()-0.5) * w, (random_f32()-0.5) * h)
}