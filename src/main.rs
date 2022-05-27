use std::error::Error;
use xcb::x;
use xcb::Event;

fn main() -> Result<(), Box<dyn Error>> {
    let (conn, screen) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen as usize).unwrap();

    let root = screen.root();

    conn.send_request(&x::ChangeWindowAttributes {
        window: root,
        value_list: &[x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_NOTIFY)]
    });

    conn.send_request(&x::GrabKey {
        grab_window: root,
        key: 11,
        keyboard_mode: x::GrabMode::Async,
        modifiers: x::ModMask::ANY,
        owner_events: false,
        pointer_mode: x::GrabMode::Async
    });

    conn.flush()?;

    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                std::process::Command::new("alacritty").spawn()?;
                println!("{}", ev.detail());
            }

            _ => ()
        }
    }

    Ok(())
}
