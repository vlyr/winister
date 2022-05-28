use xcb::x::Event;

pub fn event_to_string(event: &Event) -> &'static str {
    match event {
        Event::KeyPress(_) => "key_press",
        Event::MapNotify(_) => "map_notify",
        _ => "other"
    }
}
