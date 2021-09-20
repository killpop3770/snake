extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::{GlutinWindow as Window};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
#[allow(non_snake_case)]
#[allow(unused_imports)]
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::clear;
use piston::{EventLoop, ButtonEvent, ButtonState, Button, Key};

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    snake: Snake,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let green: [f32; 4] = [0.0, 1.0, 0.0, 0.5];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(green, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
            if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
            if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
            if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
            if last_direction != Direction::Left => Direction::Right,

            _ => last_direction
        };
    }
}

struct Snake {
    body: LinkedList<(f32, f32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {

        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 20 as f32) as f64,
                    (y * 20 as f32) as f64,
                    20_f64)
            }).collect();


        gl.draw(args.viewport(), |_c, gl| {
            let transform = _c.transform;

            squares.into_iter().for_each(
                |square|
                    graphics::rectangle(red, square, transform, gl)
            );
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Right => new_head.0 += 1 as f32,
            Direction::Left => new_head.0 -= 1 as f32,
            Direction::Up => new_head.1 -= 1 as f32,
            Direction::Down => new_head.1 += 1 as f32,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new(
        "Snaky",
        [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter(
                (vec![(0 as f32, 0 as f32), (0 as f32, 1 as f32)])
                    .into_iter()),
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(7);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            app.update();
        }

        if let Some(key) = e.button_args() {
            if key.state == ButtonState::Press {
                app.pressed(&key.button);
            }
        }
    }
}