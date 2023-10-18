use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape | Keycode::Q),
                ..
            } => return false,
            _ => {}
        }
    }
    true
}
