#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use sdl2_game_window::GameWindowSDL2;


use opengl_graphics::{
    Gl,
    Texture,
};
use graphics::*;
use piston::{
    AssetStore,
    Game,
    GameIteratorSettings,
    GameWindowSettings,
    RenderArgs,
};

pub struct App {
    gl: Gl
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App { gl: Gl::new() }
    }
}

impl Game for App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.viewport(0, 0, args.width as i32, args.height as i32);
        let ref c = Context::abs(args.width as f64, args.height as f64);
        c.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run gui on the main thread.
    native::start(argc, argv, main)
}

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );


    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}

