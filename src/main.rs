//! Graphical Representation of Clock Problems (see README.md for details)
mod shapes;
mod events;
mod clock;


use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::timer::Timer;
use sdl2::video::Window;
use sdl2::TimerSubsystem;
use std::f64::consts::PI;
use std::time::Duration;

const WINDOW_TITLE: &'static str = "Clock Problem Visualizer";

const WINDOW_WIDTH: u32 = 801;
const WINDOW_HEIGHT: u32 = 600;


fn init_sdl() -> (Canvas<Window>, EventPump, TimerSubsystem) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    return (window.into_canvas().build().unwrap(), sdl_context.event_pump().unwrap(), sdl_context.timer().unwrap());
}

fn main() {
    let (mut canvas, mut event_pump, timer) = init_sdl();
    let mut clock = clock::Clock::new(timer);
    let circle = shapes::Circle::new(Point::new((WINDOW_WIDTH/2) as i32, (WINDOW_HEIGHT/2) as i32), 100);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let bg_color = Color::RGB(249,228,183);
    loop {
        canvas.set_draw_color(bg_color);
        canvas.clear();
        if !events::handle_events(&mut event_pump) { break }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        circle.draw(&mut canvas, Some(4), Some(Color::WHITE));
        clock.update();
        let (second_vector, minute_vector, hour_vector) = clock.to_vectors(circle.radius as f64);
        let seconds_line = shapes::Line::new(Point::new(circle.center.x, circle.center.y), 
            Point::new((circle.center.x as f64 + second_vector.0) as i32, (circle.center.y as f64 + second_vector.1) as i32));
        let minutes_line = shapes::Line::new(Point::new(circle.center.x, circle.center.y), 
            Point::new((circle.center.x as f64 + minute_vector.0) as i32, (circle.center.y as f64 + minute_vector.1) as i32));
        let hour_line = shapes::Line::new(Point::new(circle.center.x, circle.center.y), 
            Point::new((circle.center.x as f64 + hour_vector.0) as i32, (circle.center.y as f64 + hour_vector.1) as i32));
        seconds_line.draw(&mut canvas).unwrap();
        minutes_line.draw(&mut canvas).unwrap();
        hour_line.draw(&mut canvas).unwrap();
        canvas.present();
    }
}
