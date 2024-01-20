use nannou::prelude::*;
use nannou::noise::*;
use rand::prelude::*;

use crate::lib::particle::Particle;
use crate::lib::sdf::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const NUMBER_OF_PARTICLES: u32 = 1_000;
const NUMBER_OF_CIRCLES: u32 = 35;
const PARTICLE_RADIUS: f32 = 10.0;

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
    noise: Perlin,
    distance: SDFUnion,
}

fn random_vec(w: f32, h: f32) -> Vec2 {
    vec2((random_f32()-0.5) * w, (random_f32()-0.5) * h)
}

fn add_particles(particles: &mut Vec<Particle>) {
    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    for _ in 0..NUMBER_OF_PARTICLES {        
        let pos = random_vec(w, h);
        let vel = random_vec(1.0, 1.0);
        let acc = random_vec(0.6, 0.6);
        particles.push(Particle::new(pos, vel, acc));
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    let mut particles: Vec<Particle> = Vec::new();
    add_particles(&mut particles);
    let mut union: Vec<Box<dyn SignedDistanceFunction>> = Vec::new();
    for _ in 0..NUMBER_OF_CIRCLES {
        union.push(Box::new(Circle::new(random_vec(w, h), random_f32() * 5.0)));
    }

    let distance = SDFUnion::new(union);
    
    Model { particles: particles, noise: Perlin::new(), distance: distance, }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update Particles
    for particle in model.particles.iter_mut() {
        let acc = model.distance.grad(particle.position);
        particle.acceleration = -acc.normalize() * particle.acceleration.length();
        particle.update(1.0);
    }
    // Update Distance Function
    let sn = 0.01;
    //let x_noise = model.noise.get([sn*model.distance.center.x as f64, sn*model.distance.center.y as f64, 0.0]);
    //let y_noise = model.noise.get([sn*model.distance.center.x as f64, sn*model.distance.center.y as f64, 1.0]);
    //model.distance.center += vec2(x_noise as f32 * 10.0, y_noise as f32 * 10.0);
    //model.distance.center += vec2(1.0, 1.0);

    /*for sdf in model.distance.sdfs.iter_mut() {
        sdf.center = sdf.center.rotate(0.003);
    }*/
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let size = 20.0;
    let n_x = (app.window_rect().w() / size) as u32;
    let n_y = (app.window_rect().h() / size) as u32;
    let div = 1.1;
    let delta_x = size * (n_x-1) as f32 * 0.5;
    let delta_y = size * (n_y-1) as f32 * 0.5;
    let gdist = f32::pow(f32::pow(app.window_rect().w()/2.0, 2.0) + f32::pow(app.window_rect().h()/2.0, 2.0), 0.5)*0.05;
    for i in 0..n_y {
        for j in 0..n_x {
            let x = j as f32 * size - delta_x;
            let y = i as f32 * size - delta_y;
            let dist = model.distance.distance(vec2(x, y)) / gdist;
            //draw.rect().x_y(x, y).w_h(size/div, size/div).color(hsla(dist, 0.6, 0.3, 0.01));
            draw.rect().x_y(x, y).w_h(size/div, size/div).color(BLACK).z(4.0);
        }
    }

    draw.rect().w_h(WIDTH as f32, HEIGHT as f32). color(srgba(0.0, 0.0, 0.0, 0.06));
    let noise = Perlin::new();
    let r = noise.get([app.elapsed_frames() as f64 * 0.003, 1.0]) * 2.0;
    let g = noise.get([app.elapsed_frames() as f64 * 0.003, 0.0]) * 2.0;
    let b = noise.get([app.elapsed_frames() as f64 * 0.003, 2.0]) * 2.0;
    //let r = f64::sin(app.elapsed_frames() as f64 * 0.1) * 3.0;
    //let g = f64::cos(app.elapsed_frames() as f64 * 0.1) * 3.0;
    //print!("{} ", r);
    for particle in model.particles.iter() {
        let dist = model.distance.distance(particle.position) / gdist;
        //draw.ellipse().xy(particle.position).w_h(PARTICLE_RADIUS, PARTICLE_RADIUS).color(hsla(dist, 0.9, 0.3, 0.01));
        draw.ellipse().xy(particle.position).w_h(PARTICLE_RADIUS, PARTICLE_RADIUS).color(rgba(dist, dist*g as f32, dist*b as f32, 0.02));
    }

    draw.to_frame(app, &frame).unwrap();

    if CAPTURE {
        captured_frame_path(app);
    }
}

fn draw_distance_field(app: &App, model: &Model) {

    let size = 20.0;
    let n_x = (app.window_rect().w() / size) as u32;
    let n_y = (app.window_rect().h() / size) as u32;
    let div = 1.3;
    let delta_x = size * n_x as f32 * 0.5;
    let delta_y = size * n_y as f32 * 0.5;
    let gdist = f32::pow(f32::pow(app.window_rect().w()/2.0, 2.0) + f32::pow(app.window_rect().h()/2.0, 2.0), 0.5);
    for i in 0..n_y {
        for j in 0..n_x {
            let x = j as f32 * size - delta_x;
            let y = i as f32 * size - delta_y;
            let dist = model.distance.distance(vec2(x, y)) / gdist;
            app.draw().rect().x_y(x, y).w_h(size/div, size/div).color(hsl(abs(dist), 0.6, 0.3));
        }
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