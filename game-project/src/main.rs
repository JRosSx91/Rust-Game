extern crate piston_window;
use piston_window::*;

struct Player {
    x: f64,
    y: f64,
    size: f64,
}
struct Enemy {
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
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);

            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [player.x, player.y, player.size, player.size],
                c.transform,
                g,
            );
        });
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => player.y -= 1.0,
                Key::Down => player.y += 1.0,
                Key::Left => player.x -= 1.0,
                Key::Right => player.x += 1.0,
                _ => {}
            }
        }
    }
}
