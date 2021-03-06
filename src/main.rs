extern crate piston_window;

use piston_window::*;

fn main() {
    struct Game {
        rotation: f64,
        x: f64,
        y: f64,
        up_d: bool,
        down_d: bool,
        left_d: bool,
        right_d: bool,
    }
    impl Game {
        fn new() -> Game {
            Game {
                rotation: 0.0,
                x: 0.0,
                y: 0.0,
                up_d: false,
                down_d: false,
                left_d: false,
                right_d: false,
            }
        }
    }

    let mut game = Game::new();
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        match e {
            Event::Update(upd) => {
                game.rotation += 3.0 * upd.dt;
                if game.up_d {
                    game.y += (-50.0) * upd.dt;
                }
                if game.down_d {
                    game.y += (50.0) * upd.dt;
                }
                if game.left_d {
                    game.x += (-50.0) * upd.dt;
                }
                if game.right_d {
                    game.x += (50.0) * upd.dt;
                }
            }
            Event::Render(ren) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
                    let square = rectangle::square(0.0, 0.0, 100.0);
                    let red = [1.0, 0.0, 0.0, 1.0];
                    // We translate the rectangle slightly so that it's centered;
                    // otherwise only the top left corner would be centered
                    rectangle(red,
                              square,
                              center.trans(game.x, game.y)
                                  .rot_rad(game.rotation)
                                  .trans(-50.0, -50.0),
                              g);
                });
            }
            Event::Input(inp) => {
                match inp {
                    Input::Press(but) => {
                        match but {
                            Button::Keyboard(Key::Up) => {
                                game.up_d = true;
                            }
                            Button::Keyboard(Key::Down) => {
                                game.down_d = true;
                            }
                            Button::Keyboard(Key::Left) => {
                                game.left_d = true;
                            }
                            Button::Keyboard(Key::Right) => {
                                game.right_d = true;
                            }
                            _ => {}
                        }
                    }
                    Input::Release(but) => {
                        match but {
                            Button::Keyboard(Key::Up) => {
                                game.up_d = false;
                            }
                            Button::Keyboard(Key::Down) => {
                                game.down_d = false;
                            }
                            Button::Keyboard(Key::Left) => {
                                game.left_d = false;
                            }
                            Button::Keyboard(Key::Right) => {
                                game.right_d = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

    }
}
