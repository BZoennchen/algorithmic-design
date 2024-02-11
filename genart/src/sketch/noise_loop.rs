use nannou::prelude::*;
use nannou::noise::*;
use rand::prelude::*;
use std::f32::consts::PI;

use genart::prelude::*;
use genart::random::noise::*;

const CAPTURE: bool = true;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 780;

const NUMBER_OF_THINGS: u32 = 10;

pub fn main() {
       nannou::app(model)
        .update(update)
        .run();
}

struct Thing {
    x_center: f32, y_center: f32, radius: f32, std: f32, seed: f32, noise: Noise<OpenSimplex>,
}

impl Thing {

    fn val(&self, t: f32, seed: f32) -> f32 {
        let v1 = self.std*(TWO_PI * t).cos() + seed;
        let v2 = self.std*(TWO_PI * t).sin() + seed;
        self.noise.noise2d(v1, v2)
    }

    fn x(&self, t: f32) -> f32 {
        let val = self.val(t, self.seed);
        self.x_center + val * self.radius
    }

    fn y(&self, t: f32) -> f32 {
        let val = self.val(t, self.seed + 33.0);
        self.y_center + val * self.radius
    }
}

impl PartialEq for Thing {
    fn eq(&self, other: &Self) -> bool {
        self.x_center == other.x_center && self.y_center == other.y_center && self.radius == other.radius && self.std == other.std && self.seed == other.seed
    }
}

struct Model { things: Vec<Thing>, }

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
    let radius = 300.0;
    let things = (0..NUMBER_OF_THINGS).map(|i| {
        
        let x_center = radius * (i as f32 / NUMBER_OF_THINGS as f32 * TWO_PI).cos();
        let y_center = radius * (i as f32 / NUMBER_OF_THINGS as f32 * TWO_PI).sin();
        
        let noise = Noise::default_simplex();
        let thing = Thing { 
            x_center: x_center, 
            y_center: y_center, 
            radius: random_range(50.0, 70.0), 
            std: random_range(0.6, 1.0),
            seed: random_range(0., 1000.),
            noise: noise,
        };
        thing
    }).collect();

    Model { things }
}


fn update(app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame){
    let time = app.duration.since_start.as_secs_f32();
    let draw = app.draw();
    let window_rect = app.window_rect();
    let t = time * 0.15;

    draw.rect().xy(window_rect.xy()).wh(window_rect.wh()).color(rgba(0.0, 0.0, 0.0, 0.9));
    
    let delay = 1.0;
    for i1 in (0..model.things.len()) {
        let thing1 = model.things.get(i1).unwrap();
        for i2 in (i1+1..model.things.len()) {
            let thing2 = model.things.get(i2).unwrap();
            let m = 1000;
            let points = (0..=m).map(|i| {
                let tt = i as f32 / m as f32;
                let x = lerp(thing1.x(t - delay*tt), thing2.x(t - delay*(1.0-tt)), tt);
                let y = lerp(thing1.y(t - delay*tt), thing2.y(t - delay*(1.0-tt)), tt);
            
                ((x,y), rgba(1.0, 1.0, 1.0, 0.5*(tt.max(1.0-tt)-0.49)))
            });


            draw.polyline().weight(1.0).points_colored(points);
        }
        draw.ellipse().x_y(thing1.x(t), thing1.y(t)).w_h(7.0, 7.0).color(rgba(1.0, 1.0, 1.0, 1.0));
    }

    if CAPTURE && t < 1.0 {
        captured_frame_path(app);
    }
    draw.to_frame(app, &frame).unwrap();
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