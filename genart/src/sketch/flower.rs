use nannou::prelude::*;
use nannou::noise::*;

use crate::lib::particle::Particle;
use crate::lib::sdf::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const NUMBER_OF_PARTICLES: u32 = 100;
const PARTICLE_RADIUS: f32 = 3.0;
const CAPTURE: bool = false;

type Color = (f32, f32, f32, f32,);

pub fn main() {
       nannou::app(model)
        .update(update)
        .run();
}


struct Model {
    particles: Vec<(Particle, Color)>,
    noise: Perlin,
}

fn random_vec(w: f32, h: f32) -> Vec2 {
    vec2((random_f32()-0.5) * w, (random_f32()-0.5) * h)
}

fn add_particles(particles: &mut Vec<(Particle, Color)>, n: u32, vel: Vec2, color: Color) {
    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    for _ in 0..n {        
        let pos = vec2(0.0, 0.0);
        let vel = vel;
        let acc = -vel * 0.0016;
        particles.push((Particle::new(pos, vel, acc), color));
    }
}

fn model(app: &App) -> Model {
        let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model { particles: Vec::new(), noise: Perlin::new() }
}

fn to_color(app: &App, model: &Model) -> Color {
    let r = model.noise.get([1.0, app.elapsed_frames() as f64 * 0.01]);
    let g = model.noise.get([0.0, app.elapsed_frames() as f64 * 0.01]);
    return (r as f32, g as f32, 0.3, 0.6);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Remove Particles
    model.particles.retain(|(particle,_)| {
        particle.velocity.length() >= 0.01 && (
        particle.position.x <= WIDTH as f32 / 2.0 &&
        -particle.position.x <= WIDTH as f32 / 2.0 &&
        particle.position.y <= HEIGHT as f32 / 2.0 &&
        -particle.position.y <= HEIGHT as f32 / 2.0)
    });

    let angle = app.elapsed_frames() as f32 * 0.003;
    let vel = vec2(angle.cos(), angle.sin()).normalize() * random_range(0.0, 1.0);
    let color = to_color(app, model);

    let sn = 0.01;
    let beta = angle as f32 * 0.04;
    let h = model.noise.get([app.elapsed_frames() as f64* sn, 1.0]) as f32 * 0.001;
    let h = beta.cos() + h;
    add_particles(&mut model.particles, 1, vel, (h, 0.4, 0.4, 0.4));

    // Update Particles
    for (particle, _) in model.particles.iter_mut() {
        particle.update(1.0);
        particle.velocity = particle.velocity.rotate(f32::PI() * (-random_f32()) * 0.001);
        particle.acceleration = -particle.velocity.normalize() * particle.acceleration.length();
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let h = app.window_rect().h();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    //draw.rect().wh(app.window_rect().wh()). color(srgba(0.0, 0.0, 0.0, 0.0009));
    for (particle, color) in model.particles.iter() {
        draw.ellipse().xy(particle.position).w_h(PARTICLE_RADIUS, PARTICLE_RADIUS).color(hsla(0.8, 0.9, 0.6-particle.position.length() / h, 0.01 * particle.position.length()/h));
        //draw.ellipse().xy(particle.position).w_h(PARTICLE_RADIUS, PARTICLE_RADIUS).color(hsla(color.0, 0.9, 0.6-particle.position.length() / h, 1.0));
    }

    draw.to_frame(app, &frame).unwrap();

    if CAPTURE {
        captured_frame_path(app);
    }
    
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            captured_frame_path(app);
        }
        _other => {}
    }
}

fn captured_frame_path(app: &App) {
    let path = app.project_path()
        .expect("failed to locate `project_path`")
        .join("capture")
        //.join(app.exe_name().unwrap())
        .join(format!("snapshot{:06}", app.elapsed_frames()))
        .with_extension("png");

    app.main_window().capture_frame(path);    
}