extern crate piston_window;
use piston_window::*;

struct Player {
    x: f64,
    y: f64,
    size: f64,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("JRosSx91 Game!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _| {
            clear([1.0; 4], g);
        });
    }
}
