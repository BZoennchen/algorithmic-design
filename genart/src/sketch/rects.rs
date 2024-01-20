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

fn draw_rect(draw: &Draw, x: f32, y: f32, size: f32, color: Rgba8) {
    /*draw.rect()
        .w_h(size, size)
        .x_y(x, y)
        .color(color);*/


    let mut c = color.clone();
    c.red -= thread_rng().gen_range(0..15);
    c.green -= thread_rng().gen_range(0..15);
    c.blue -= thread_rng().gen_range(0..15);

    /*draw.rect()
        .w_h(size, size)
        .x_y(x, y)
        .color(c);*/
    let padding = 2.0;
    let color_distortion = 25;
    let sw = 1.0;
    for _ in 0..120 {
        let mut d = color.clone();
        d.alpha = 150;
        d.red -= u8::min(d.alpha, thread_rng().gen_range(0..color_distortion));
        d.green -= u8::min(d.alpha, thread_rng().gen_range(0..color_distortion));
        d.blue -= u8::min(d.alpha, thread_rng().gen_range(0..color_distortion));
        let vertices = (0..3).map(|i| {
            pt2(thread_rng().gen_range(x-size/2.0+padding..x+size/2.0-padding), thread_rng().gen_range(y-size/2.0+padding..y+size/2.0-padding))
        });
        draw.polyline().weight(sw).join_round().points(vertices).color(d);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let time = app.duration.since_start.as_secs_f32();
    let mut draw = app.draw();
    let noise = Perlin::new();

    let colors = [
        rgba8(230, 164, 180, 255), 
        rgba8(243, 215, 202, 255), 
        rgba8(255, 248, 227, 255), 
        rgba8(245, 238, 230, 255)];

    if app.elapsed_frames() == 0 {
        // rgb8(128, 188, 189)
        // rgb8(51, 41, 65)
        draw.background().color(rgb8(22, 26, 48));

        let size = 45.0;
        let cols = (app.window_rect().w() as f32 / size) as i32 + 2;
        let rows = (app.window_rect().h() as f32 / size) as i32 + 2;
        draw = draw.translate(vec3(-app.window_rect().w() as f32 / 2.0, -app.window_rect().h() as f32 / 2.0, 0.0));
        for j in 0..rows {
            for i in 0..cols {
                let x = i as f32 * size;
                let y = j as f32 * size;
                draw_rect(&draw, x, y, size, colors[random_range(0, 4)]);
            }
        }
    }
    
    //draw.rect().wh(app.window_rect().wh()).color(rgba(1.0, 1.0, 1.0, 0.1));

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