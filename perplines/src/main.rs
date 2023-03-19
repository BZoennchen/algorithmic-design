use nannou::prelude::*;
use nannou::rand;
use rand_distr::{Normal, Distribution};

const SIZE: u32 = 2160;
const SIZE_W: u32 = 4096;
const SPLIT_PROB: f32 = 0.07;
const SPLIT_DECAY_RATE: f32 = 0.3;
const MAG_DECAY_RATE: f32 = 0.76;
const MIN_MAG_RATIO: f32 = 0.1;
const DELTA: f32 = 0.1;
const SPEED: f32 = 0.5;
const MIN_PARTS: u16 = 3;
const MAX_PARTS: u16 = 22;
const SIZE_RATIO: f32 = 0.1;
const STROKE_WIDTH: f32 = 0.5;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    branches: Vec<Branch>,
    parts: u16,
}

struct Branch {
    start: Vec2,
    end: Vec2,
    final_end: Vec2,
    mag: f32,
    dir: Vec2,
    split_prob: f32,
}

impl Branch {
    fn new(start: Vec2, end: Vec2, final_end: Vec2, split_prob: f32) -> Self {
        let mag = final_end.distance(start);
        let dir = (final_end - start).normalize();
        Branch { 
            start, 
            end, 
            final_end,
            mag,
            dir,
            split_prob,
        }
    }

    fn root(len: f32) -> Self {
        Self::new(
            Vec2::ZERO,
            Vec2::ZERO, 
            vec2(len, len), 
            SPLIT_PROB
        )
    }

    fn update(&mut self, delta: f32) -> Option<Branch> {
        let mut child = None;
        if !self.has_finished() {
            self.grow(delta);
            if random_f32() < &self.split_prob * delta {
                child = Some(self.split());
            }
        }
        child
    }

    fn grow(&mut self, delta: f32) {
        self.end += self.dir * delta;
        if self.has_finished() {
            self.end = self.final_end;
        }        
    }

    fn split(&mut self) -> Branch {
        let child_dir = self.dir.rotate(-TAU / 4.0);
        let mean = self.mag * MAG_DECAY_RATE;
        let dist = Normal::new(mean, 1.0).unwrap();
        let mut child_mag = dist.sample(&mut rand::thread_rng()) * self.mag;
        child_mag = child_mag.min(self.mag - self.start.distance(self.end));
        child_mag = child_mag.max(SIZE as f32 * MIN_MAG_RATIO);

        let child_start = self.end;
        let child_final_end = child_start + child_dir * child_mag;

        Branch::new(
            child_start, 
            child_start, 
            child_final_end, 
            self.split_prob * SPLIT_DECAY_RATE)
    }

    fn has_finished(&self) -> bool {
        self.start.distance(self.end) >= self.mag
    }
}

fn scale(scale: f32) -> f32 { 
    SIZE as f32 * scale
}

fn init_model() -> Model {
    let mut branches = Vec::new();
    branches.push(Branch::root(scale(SIZE_RATIO)));
    let parts = random_range(MIN_PARTS, MAX_PARTS);
    Model {
        branches,
        parts,
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(SIZE_W, SIZE)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    init_model()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut new_branches = Vec::new();
    let delta = SPEED;
    for _ in 0..(delta/DELTA).ceil() as i32 {
        for branch in model.branches.iter_mut() {
            let child = branch.update(DELTA);
            match child {
                Some(x) => { new_branches.push(x)},
                None => {}
            }
        }
    }

    model.branches.append(&mut new_branches);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            let new_model = init_model();
            model.parts = new_model.parts;
            model.branches = new_model.branches;
        }
        Key::S => {
            app.main_window().capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //let background_color = rgb(40.0/255.0, 40.0/255.0, 40.0/255.0);
    let background_color = rgb(0.0, 0.0, 0.0);
    let my_red = rgba(252.0/255.0, 85.0/255.0, 85.0/255.0, 0.075);
    
    draw.background().color(background_color);
    
    for i in 1..(model.parts+1) {
        let angle_delta = TAU / model.parts as f32 * i as f32;

        for branch in model.branches.iter() {
            draw.line()
                .start(branch.start)
                .end(branch.end)
                .stroke_weight(STROKE_WIDTH)
                .color(my_red)
                .rotate(angle_delta);
        }
        
        let rdraw = draw.scale_y(-1.0);
        for branch in model.branches.iter() {
            rdraw.line()
                .start(branch.start)
                .end(branch.end)
                .stroke_weight(STROKE_WIDTH)
                .color(my_red)
                .rotate(angle_delta);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    /*let n_frame = app.elapsed_frames();
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);*/
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(app.exe_name().unwrap())
        .join(format!("snapshot{:06}", frame.nth()))
        .with_extension("png")
}