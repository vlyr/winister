use xcb::x::Event;
use std::fs::OpenOptions;
use std::io::Write;

pub fn event_to_string(event: &Event) -> &'static str {
    match event {
        Event::KeyPress(_) => "key_press",
        Event::MapNotify(_) => "map_notify",
        Event::DestroyNotify(_) => "destroy_notify",
        Event::ButtonPress(_) => "button_press",
        _ => "other"
    }
}

pub fn debug(data: String) {
    let home = std::env::var("HOME").unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .read(true)
        .create(true)
        .open(format!("{}/.local/share/winister-debug", home))
        .unwrap();

    file.write(data.as_bytes()).unwrap();
}
