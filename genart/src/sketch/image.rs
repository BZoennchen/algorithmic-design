use nannou::prelude::*;
use nannou::noise::*;
use nannou::image::io::Reader as ImageReader;
use nannou::image::open;
use nannou::image::GrayImage;
use nannou::image::buffer::ConvertBuffer;
use nannou::image::Pixel;
use rand::prelude::*;

const CAPTURE: bool = false;

pub fn main() {
       nannou::app(model)
        .update(update)
        .run();
}

struct Model { pixels: Vec<u8>, cols: usize, rows: usize }

fn model(app: &App) -> Model {
    
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("helena.jpeg");
    let image = open(img_path).unwrap().to_rgba8();
    let gray_image: GrayImage = image.convert();

    let (cols, rows) = gray_image.dimensions();
    let cols = cols as usize;
    let rows = rows as usize;
    let mut pixels: Vec<u8> = Vec::new();

    for pixel in gray_image.pixels() {
        let gray_val = (pixel.0[0]);
        pixels.push(gray_val);
    }

    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size((cols / 3) as u32, (rows / 3) as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model { pixels, cols, rows }
}


fn update(app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame){
    let time = app.duration.since_start.as_secs_f32();
    let mut draw = app.draw();

    let rect = app.window_rect();

    let rows: usize = 100;
    let cols: usize = (rows as f32 * (model.cols as f32 / model.rows as f32)) as usize;
    let col_scale: usize = model.cols / cols;
    let row_scale: usize = model.rows / rows;

    let size = (rect.w() / cols as f32).max(rect.h() / rows as f32);
    draw = draw.scale_y(-1.0);
    draw = draw.translate(vec3(-rect.w() / 2.0 + size / 2.0, -rect.h() / 2.0 + size / 2.0, 0.0));

    for col in 0..cols {
        for row in 0..rows {
            let index: usize = col * col_scale + row * row_scale * model.cols;
            let pixel: u8 = model.pixels[index];
            let x = col as f32 * size;
            let y = row as f32 * size;
            draw.rect().x_y(x,y).w_h(size, size).color(rgb8(pixel, pixel, pixel));
        }
    }

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