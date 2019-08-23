enum Event {
    Load,
    Unload,
    KeyPressed(char),
    MouseClicked { x: i32, y: i32 },
}

fn event_handler(e: Event) {
    match e {
        Event::Load => println!("Loaded"),
        Event::Unload => println!("Unloaded"),
        Event::KeyPressed(c) => println!("User pressed {}.", c),
        Event::MouseClicked { x, y } => println!("User clicked ({}, {})", x, y),
    }
}
fn main() {
    let pressed = Event::KeyPressed('e');
    let click   = Event::MouseClicked { x: 42, y: 420 };
    let load    = Event::Load;
    let unload  = Event::Unload;

    event_handler(pressed);
    event_handler(click);
    event_handler(load);
    event_handler(unload);
}
