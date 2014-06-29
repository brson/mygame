#![feature(globs)]
#![allow(unused_imports, dead_code, unused_variable)]

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

static NUM_TONES: uint = 12;

pub struct App {
    gl: Gl,

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
        c.rgb(0.0, 0.0, 0.0).draw(&mut self.gl);

        for i in range(0, NUM_TONES) {
            let tone = ToneOff(i);
            draw_tone(c, &mut self.gl, args, tone);
        }
    }
}

enum ToneState {
    ToneOff(uint),
    ToneOn(uint, bool, bool, bool)
}

fn draw_tone(cx: &Context, gl: &mut Gl, args: &RenderArgs, tone: ToneState) {

    let tone_num = match tone { ToneOff(i) | ToneOn(i, _, _, _) => i };

    let width = args.width as f64;
    let height = args.height as f64;

    let unit = width / (NUM_TONES as f64 + 2.0);
    let subunit = unit / 7.0;
    let border = 0.025 * unit;

    let x = unit * (tone_num as f64 + 1.0);
    let y = 1.0 * unit;
    let d = 1.0 * unit;
    let color = Color { r: 1.0 / 5.0, g: 1.0 / 5.0, b: 1.0 / 5.0 };

    draw_circle(cx, gl, x, y, d, color, border);

    let x = x + subunit;
    let y = y + subunit;
    let d = d - subunit * 2.0;
    let color = Color { r: 1.0 / 2.0, g: 1.0 / 2.0, b: 1.0 / 2.0 };

    draw_circle(cx, gl, x, y, d, color, border);

    let x = x + subunit;
    let y = y + subunit;
    let d = d - subunit * 2.0;
    let color = Color { r: 1.0, g: 1.0, b: 1.0 };

    draw_circle(cx, gl, x, y, d, color, border);
}

struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

fn draw_circle(cx: &Context, gl: &mut Gl,
               x: f64, y: f64, d: f64, color: Color, border: f64) {
    let big_x = x;
    let big_y = y;
    let big_diameter = d;
    let big_x_ = x + border;
    let big_y_ = y + border;
    let big_diameter_ = d - border * 2.0;

    cx.ellipse(big_x, big_y, big_diameter, big_diameter)
        .rgb(color.r, color.g, color.b)
        .draw(gl);
    cx.ellipse(big_x_, big_y_, big_diameter_, big_diameter_)
        .rgb(0.0, 0.0, 0.0)
        .draw(gl);
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
            size: [720, 480],
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

