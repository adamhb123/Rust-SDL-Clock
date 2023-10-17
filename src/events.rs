use sdl2::{EventPump, event::Event, keyboard::Keycode};

pub fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown {
                keycode: Some(Keycode::Escape | Keycode::Q),
                ..
            } => return false,
            _ => {}
        }
    }
    true
}