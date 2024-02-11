use nannou::prelude::*;
use nannou::noise::*;
use rand::prelude::*;

const CAPTURE: bool = false;

pub fn main() {
       nannou::app(model)
        .update(update)
        .run();
}

struct Model { }

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(1280, 720)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model { }
}


fn update(app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame){
    let time = app.duration.since_start.as_secs_f32();
    let draw = app.draw();
    

    if CAPTURE {
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