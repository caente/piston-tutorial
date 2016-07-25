extern crate piston_window;

use piston_window::*;

fn main() {

	struct Game {
		rotation : f64
	}

	impl Game {
		fn new() -> Game{
			Game{rotation : 0.0}
		}
	}
 	let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .exit_on_esc(true).build().unwrap();
	let mut game = Game::new();
    while let Some(e) = window.next() {
		match e {
			Event::Update(udp) => {
				game.rotation += 3.0 * udp.dt;
			}
			Event::Render(ren) => {
                window.draw_2d(&e, |c,g| { 
                    clear([0.0, 0.0, 0.0, 1.0], g);
		            let center = c.transform.trans(300.0, 300.0);
		            let square = rectangle::square(0.0, 0.0, 100.0);
		            let red = [1.0,0.0,0.0,1.0];
		            rectangle(red, square, center.rot_rad(game.rotation).trans(-50.0, -50.0), g);    
                });
			}
            Event::Input(inp) => {}
			_ => {}
		}

    }
}
