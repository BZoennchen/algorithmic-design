use nannou::prelude::*;
use nannou::noise::*;

use genart::core::particle::Particle;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const NUMBER_OF_PARTICLES: u32 = 100;
const PARTICLE_RADIUS: f32 = 2.0;
const CAPTURE: bool = false;

type Color = (f32, f32, f32, f32,);

pub fn main() {
       nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    particles: Vec<Particle>,
}

fn add_particles(particles: &mut Vec<Particle>, vel: Vec2, color: Color) {    
    let pos = vec2(0.0, 0.0);
    let vel = vel;
    let acc = -vel * 0.0016;
    particles.push(Particle::new(pos, vel, acc));
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model { particles: Vec::new(), }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new();
    // Remove Particles
    model.particles.retain(|particle| {
        particle.velocity.length() >= 0.01 && (
        particle.position.x <= WIDTH as f32 / 2.0 &&
        -particle.position.x <= WIDTH as f32 / 2.0 &&
        particle.position.y <= HEIGHT as f32 / 2.0 &&
        -particle.position.y <= HEIGHT as f32 / 2.0)
    });

    let angle = app.elapsed_frames() as f32 * 0.03;
    let vel = vec2(angle.cos(), angle.sin()).normalize() * random_range(0.0, 1.0);

    let sn = 0.01;
    let beta = angle as f32 * 0.4;
    let h = noise.get([app.elapsed_frames() as f64* sn, 1.0]) as f32 * 0.001;
    let h = beta.cos() + h;
    add_particles(&mut model.particles, vel, (h, 0.4, 0.4, 0.4));

    // Update Particles
    for particle in model.particles.iter_mut() {
        particle.update(1.0);
        particle.velocity = particle.velocity.rotate(f32::PI() * (-random_f32()) * 0.002);
        particle.acceleration = -particle.velocity.normalize() * particle.acceleration.length();
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let mut draw = app.draw();
    let h = app.window_rect().h();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw = draw.translate(vec3(0.0, 0.0, 0.0));
    for particle in model.particles.iter() {
        let gray = 5.0 * particle.position.length() / h;
        draw.ellipse().xy(particle.position).w_h(PARTICLE_RADIUS, PARTICLE_RADIUS).color(rgba(gray, gray, gray, 0.01));
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