enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page is loading..."),
        WebEvent::PageUnload => println!("Page is unloading..."),
        WebEvent::KeyPress(c) => println!("Key press: {}", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\"", s),
        WebEvent::Click {x, y} => println!("Clicked @ x={}, y={}", x, y),
    }
}

fn main() {
    // println!("Hello, world!");

    let pressed = WebEvent::KeyPress('x');
    // to_string -> convert &str => String
    // to_owned -> owned the &str thus converting it to String
    // on previous rust version, to_owned is faster than to_string
    let pasted = WebEvent::Paste("My text".to_owned());
    let click = WebEvent::Click { x: 10, y: 20 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
