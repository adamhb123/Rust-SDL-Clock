use std::f64::consts::PI;

use sdl2::TimerSubsystem;

pub struct Clock {
    /// Twelve hour clock
    timer: TimerSubsystem,
    seconds: f64,
    minutes: f64,
    hours: f64,
}
impl Clock {
    pub fn new(timer: TimerSubsystem) -> Self {
        Clock {
            timer,
            seconds: 0.0,
            minutes: 0.0,
            hours: 0.0,
        }
    }
    pub fn update(&mut self) {
        self.seconds = (self.timer.ticks() as f64) / (60.0 * 1000.0);
        self.minutes = (self.timer.ticks() as f64) / (60.0 * 60.0 * 1000.0);
        self.hours = (self.timer.ticks() as f64) / (12.0 * 60.0 * 60.0 * 1000.0);
    }
    pub fn to_vectors(&self, magnitude: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        let second_vector = (
            magnitude * f64::cos(self.seconds * PI - PI / 2.0),
            magnitude as f64 * f64::sin(self.seconds * PI - PI / 2.0),
        );
        let minute_vector = (
            magnitude * f64::cos(self.minutes * PI - PI / 2.0),
            magnitude as f64 * f64::sin(self.minutes * PI - PI / 2.0),
        );
        let hour_vector = (
            magnitude * f64::cos(self.hours * PI - PI / 2.0),
            magnitude as f64 * f64::sin(self.hours * PI - PI / 2.0),
        );
        return (second_vector, minute_vector, hour_vector);
    }
}
