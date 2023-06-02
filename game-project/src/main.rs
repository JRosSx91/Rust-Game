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

    let mut player = Player {
        x: 0.0,
        y: 0.0,
        size: 50.0,
    };

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _| {
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // color rojo
                [player.x, player.y, player.size, player.size],
                c.transform,
                g,
            );
        });
    }
}
