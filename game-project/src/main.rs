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
    speed_x: f64,
    speed_y: f64,
}

fn main() {
    let mut score: i32 = 0;

    let mut window: PistonWindow = WindowSettings::new("JRosSx91 Game!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut player = Player {
        x: 0.0,
        y: 0.0,
        size: 50.0,
    };

    let mut enemies = vec![
        Enemy {
            x: 100.0,
            y: 100.0,
            size: 50.0,
            speed_x: 1.0,
            speed_y: 1.0,
        },
        Enemy {
            x: 200.0,
            y: 200.0,
            size: 50.0,
            speed_x: -1.0,
            speed_y: -1.0,
        },
        // puedes añadir tantos enemigos como quieras
    ];

    while let Some(e) = window.next() {
        // Actualizamos la posición de los enemigos según su velocidad.
        for enemy in &mut enemies {
            enemy.x += enemy.speed_x;
            enemy.y += enemy.speed_y;
        }

        // Hacemos que los enemigos reboten en las paredes cambiando su dirección cuando alcanzan el borde de la pantalla.
        for enemy in &mut enemies {
            if enemy.x < 0.0 || enemy.x > 640.0 - enemy.size {
                enemy.speed_x = -enemy.speed_x;
            }
            if enemy.y < 0.0 || enemy.y > 480.0 - enemy.size {
                enemy.speed_y = -enemy.speed_y;
            }
        }
        for enemy in &enemies {
            if are_colliding(&player, enemy) {
                player.x = 0.0;
                player.y = 0.0;
            }
        }
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);

            rectangle(
                [1.0, 0.0, 0.0, 1.0], //rojo
                [player.x, player.y, player.size, player.size],
                c.transform,
                g,
            );

            for enemy in &enemies {
                rectangle(
                    [0.0, 0.0, 1.0, 1.0], //azul
                    [enemy.x, enemy.y, enemy.size, enemy.size],
                    c.transform,
                    g,
                );
            }
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
fn are_colliding(a: &Player, b: &Enemy) -> bool {
    let distance_x = (b.x - a.x).abs();
    let distance_y = (b.y - a.y).abs();

    distance_x < a.size / 2.0 + b.size / 2.0 && distance_y < a.size / 2.0 + b.size / 2.0
}
