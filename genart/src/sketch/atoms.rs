use nannou::prelude::*;
use nannou::noise::*;
use rand::prelude::*;

use genart::core::particle::Particle;
use genart::core::sdf::*;
use genart::prelude::*;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

const NUMBER_OF_PARTICLES: u32 = 10_00;
const PARTICLE_RADIUS: f32 = 15.0;
const PARTICLE_MAX_VELOCITY: f32 = 10.0;

const NUMBER_OF_CIRCLES: u32 = 7;
const CIRCLE_MAX_RADIUS: f32 = 250.0;

const ACCELERATION: f32 = 1.0;
const SPEED: f32 = 1.0;
const CAPTURE: bool = false;

pub fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    particles: Vec<Particle>,
    distance: SDFUnion,
}

fn random_vec(w: f32, h: f32) -> Vec2 {
    vec2((random_f32()-0.5) * w, (random_f32()-0.5) * h)
}

fn add_particles(particles: &mut Vec<Particle>) {
    for _ in 0..NUMBER_OF_PARTICLES {        
        let mut particle = Particle::from_position(random_vec(WIDTH, HEIGHT), PARTICLE_MAX_VELOCITY);
        particle.velocity = random_vec(1.0, 1.0);
        particle.acceleration = random_vec(1.0, 1.0);
        particles.push(particle);
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut particles: Vec<Particle> = Vec::new();
    add_particles(&mut particles);

    let mut union: Vec<Box<dyn SignedDistanceFunction>> = Vec::new();
    for _ in 0..1 {
        union.push(Box::new(Circle::new(vec2(0.0, 0.0), CIRCLE_MAX_RADIUS)));
    }

    let distance = SDFUnion::new(union);
    
    Model { particles: particles,  distance: distance, }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = 0.5;

    for particle in model.particles.iter_mut() {
        let grad = model.distance.grad(particle.position);
        
        particle.acceleration -= grad * 1.0 * dt;
        particle.update(dt);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(WIDTH, HEIGHT).color(rgba(0.0, 0.0, 0.0, 0.01));

    for particle in model.particles.iter() {
        let mut dist = model.distance.distance(particle.position);
        //let t = ((dist.abs() / 200.0).cos() * 2.0);
        dist = dist.abs() / WIDTH * 2.0;
        dist = dist.sin();
        //dist = smoothstep(0.0, 0.1, dist) ;
        //dist = 0.06 / dist;
        draw.ellipse().xy(particle.position).w_h(PARTICLE_RADIUS, PARTICLE_RADIUS).color(colors::red_green(dist, 1.0));
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