use nannou::prelude::*;
use nannou_osc as osc;
use std::string::ToString;

type Sender = osc::Sender<osc::Connected>;
type Color = nannou::color::Alpha<nannou::prelude::rgb::Rgb<nannou::color::encoding::Srgb, f32>, f32>;

const NUMBER_OF_NOTES: usize = 21;
const MAX_RADIUS: f32 = 400.0;
const OSC_REC_PORT: i32 = 6448;
const LINE_WEIGHT: f32 = 0.6;
const MIN_SPEED: f32 = 100.0;
const BACKGROUND_COLOR: (f32, f32, f32, f32) = (255.0/255.0, 211.0/255.0, 182.0/255.0, 1.0);
const NOTE_COLOR: (f32, f32, f32, f32) = (255.0/255.0, 139.0/255.0, 148.0/255.0, 1.0);
//const CIRCLE_COLOR: (f32, f32, f32, f32) = (223.0/255.0, 211.0/255.0, 195.0/255.0, 1.0);
const CIRCLE_COLOR: (f32, f32, f32, f32) = (133.0/255.0, 88.0/255.0, 111.0/255.0, 1.0);
const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 1024.0;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIDTH as u32, HEIGHT as u32)
        .run();
}

fn tuple_to_color(tuple: (f32, f32, f32, f32)) -> Color {
    rgba(tuple.0, tuple.1, tuple.2, tuple.3)
}

trait Nannou {
    fn view(&self, draw: &Draw);
    fn update(&mut self, time: f32);
}

type Rgb = Srgb<u8>;

#[derive(Clone)]
struct Note {
    alpha: f32,
    start_rad: f32,
    speed: f32,
    id: i32,
    line: Line,
    rad: f32,
    round: i32,
    play_time: f32,
    play_duration: f32,
    play: bool,
    osc_addr: String,
}

impl Note {
    fn from(alpha: f32, start_rad: f32, speed: f32, id: i32, line: Line) -> Self {
        let osc_addr = "/play/midi_note".to_string();
        Self {alpha, start_rad, speed, id, line, rad: start_rad, round: 0, play_time:0.0, play_duration: 1.0, play: false, osc_addr: osc_addr}
    }

    fn play(&mut self, sender: &Sender) {
        if self.play {
            let args = vec![osc::Type::Int(self.id), osc::Type::Int(NUMBER_OF_NOTES as i32)];
            let packet = (&self.osc_addr, args);
            sender.send(packet).ok();
            self.play = !self.play;
        }
    }

    fn center(&self) -> (f32, f32) {
        (self.alpha.cos() * self.rad, self.alpha.sin() * self.rad)
    }

}

impl Nannou for Note {
    fn view(&self, draw: &Draw) {
        let mut c = tuple_to_color(CIRCLE_COLOR);
        c.alpha = (self.play_duration+0.35).min(0.4);
        draw.line().start(self.line.start).end(self.line.end).stroke_weight(self.play_duration*5.0).color(c).z(0.0);

        let center = self.center();
        let draw = draw.x(center.0).y(center.1);
        let mut c = tuple_to_color(NOTE_COLOR);
        c.alpha = 0.40;
        let mut c2 = tuple_to_color(NOTE_COLOR);
        c2.alpha = self.play_duration+0.35;
        draw.ellipse().radius(10.0).color(c).stroke_color(c2).stroke_weight(LINE_WEIGHT*2.5+self.play_duration*10.0).z(2.0);
    }

    fn update(&mut self, time: f32) {
        
        let mut sign = 1.0;
        let mut rad = self.start_rad + time * self.speed;
        let round = (rad / MAX_RADIUS) as i32;
        let mut round_mod = round;
        
        if round % 4 == 1 || round % 4 == 2 {
            sign = -1.0;
        }

        if round_mod % 2 == 1 {
            round_mod += 1;
        } 
        
        rad = rad - MAX_RADIUS * round_mod as f32;

        if self.round % 2 == 0 && round % 2 == 1 {
            self.play = true;
            self.play_time = time + 0.75;
        }
        self.rad = rad * sign;
        self.play_duration = self.play_time - time;
        self.play_duration = self.play_duration.max(0.0);
        self.round = round;

    }
}

#[derive(Clone)]
struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    fn from(start: Vec2, end: Vec2) -> Self {
        Self {start, end}
    }
}

struct Model {
    notes: Vec<Note>,
    //lines: Vec<Line>,
    sender: Sender,
}

impl Model {
    fn new() -> Self {
        let mut notes = Vec::new();
        //let mut lines = Vec::new();

        let delta = PI / NUMBER_OF_NOTES as f32;
        let speed_up = 1.0/2.0.pow(7.0) as f32;
        
        for i in 0..NUMBER_OF_NOTES {
            let speed = MIN_SPEED * (i as f32 * speed_up + 1.0);
            let alpha = delta * i as f32;
            
            let start = Vec2::new(-alpha.cos() * MAX_RADIUS, -alpha.sin() * MAX_RADIUS);
            let end = Vec2::new(alpha.cos() * MAX_RADIUS, alpha.sin() * MAX_RADIUS);
            let line = Line::from(start, end);
            //lines.push(Line::from(start, end));
            
            notes.push(Note::from(
                delta * i as f32,
                MAX_RADIUS/2.0,
                speed, 
                i as i32,
                line));
        }

        let target_addr = format!("{}:{}", "127.0.0.1", OSC_REC_PORT);
        let sender = osc::sender()
            .expect("Could not bind to default socket")
            .connect(target_addr)
            .expect("Could not connect to socket at address");

        Model { notes, sender }
    }
}

impl Model {
     fn circle(&self, draw: &Draw) {
        let mut c = tuple_to_color(CIRCLE_COLOR);
        let n = 20;
        for i in 0..n {
            let dt = LINE_WEIGHT * 2.0 * 0.7;
            c.alpha = 1.0 - i as f32 / n as f32;
            draw.ellipse().no_fill()
                .w(MAX_RADIUS*2.0-dt*i as f32)
                .h(MAX_RADIUS*2.0-dt*i as f32)
                .stroke_weight(LINE_WEIGHT*2.0)
                .stroke_color(c);

            draw.ellipse().no_fill()
                .w(MAX_RADIUS*2.0+dt*i as f32)
                .h(MAX_RADIUS*2.0+dt*i as f32)
                .stroke_weight(LINE_WEIGHT*2.0)
                .stroke_color(c);
        }
    }

    fn note_connections(&self, draw: &Draw) {
        let mut iterator = self.notes.iter();
        let mut current = iterator.next();
        
        let mut c = tuple_to_color(NOTE_COLOR);
        c.alpha = 0.5;

        loop {
            if current.is_none() {
                break;
            }
            let current_center = current.unwrap().center();
            
            let next = iterator.next();
            if next.is_some() {
                let next_center = next.unwrap().center();
                draw.line()
                    .start(Point2::new(current_center.0, current_center.1))
                    .end(Point2::new(next_center.0, next_center.1))
                    .weight(LINE_WEIGHT * 4.0)
                    .color(c);
            }

            current = next;
        }
    }
}

impl Nannou for Model {
    fn view(&self, draw: &Draw) {
        let mut c = tuple_to_color(BACKGROUND_COLOR);
        c.alpha = 0.6;
        draw.rect().w(WIDTH).h(HEIGHT).color(c);
        
        self.circle(draw);
        self.note_connections(draw);
        
        for note in self.notes.iter() {
            note.view(draw)
        }
    }
    
    fn update(&mut self, time: f32) {
        for note in self.notes.iter_mut() {
            note.update(time);
            note.play(&self.sender)
        }
    }
}

fn model(_app: &App) -> Model {
    Model::new()
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update(app.duration.since_start.as_secs_f32());
    //print!("fps: {}", app.fps());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.view(&draw);
    draw.to_frame(&app, &frame).unwrap();
}