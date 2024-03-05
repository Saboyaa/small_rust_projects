extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::piston::ButtonEvent;
use piston::Key;
use piston::ButtonState;
use piston::Button;
use crate::piston::AdvancedWindow;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  
    x: f64,
    y: f64,
    horizontal_key: f64,
    vertical_velocity: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.x / 2.0 + args.window_size[0]/2.0, self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.x += 200.0 * args.dt * self.horizontal_key;
        self.vertical_velocity += args.dt;
        self.y += 200.0*self.vertical_velocity*args.dt ;

    }
    fn button(&mut self, btn: &Button) {
        let last_vertical = self.vertical_velocity.clone();
        let last_horizontal = self.horizontal_key.clone();
        self.vertical_velocity = match btn {
            &Button::Keyboard(Key::Up) => -0.1 * f64::abs(last_vertical) + -1.0,
            &Button::Keyboard(Key::Down) => 2.0,
            _ => last_vertical,
        };
        self.horizontal_key = match btn {
            &Button::Keyboard(Key::Right) => 1.0,
            &Button::Keyboard(Key::Left) => -1.0,
            _ => last_horizontal,
        }
    }
    fn release(&mut self, btn: &Button) {
        let last_horizontal = self.horizontal_key.clone();
        self.horizontal_key = match btn {
            &Button::Keyboard(Key::Right) => 0.0,
            &Button::Keyboard(Key::Left) => 0.0,
            _ => last_horizontal,
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap()
        .set_position((0,0));    
    // Create a new game and run it.

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x: 0.0,
        y: 0.0,
        horizontal_key:0.0,
        vertical_velocity:0.0,
    };
    let mut a = 0;
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
            a += 10;
            window.set_position((a,a));
            println!("{:?},{}",window.get_position(),a)
        }

        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                app.button(&args.button);
            };
            if args.state == ButtonState::Release {
                app.release(&args.button);
            }


        }
    }
}