extern crate rand;
#[macro_use]
extern crate glium;
extern crate core;

use rand::Rng;
use std::{ f64, thread, time };
use std::vec::Vec;
use geometry::polygon::Polygon;
use geometry::vec::Vec2;
use glium::{ DisplayBuild, glutin };
use glium::glutin::{Event, MouseScrollDelta, ElementState, MouseButton, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;

pub mod geometry;
pub mod graphics;

use graphics::Graphics;

fn main() {
    let mut ctrl: Main = Main::new();
    ctrl.run();
}


struct Main {
    zoom: f64,
    center: Vec2,
    mouse_down: bool,
    mouse_pos: Vec2,
    mouse_pos_past: Vec2,
}

impl Main {
    fn run(&mut self) {
        let pqr : Polygon;
        let p = make_polygon(5, 20.0, 50.0);

        let display = glutin::WindowBuilder::new().build_glium().unwrap();
        let mut graphics = Graphics::new(&display);
        loop {
            //// Events ////
            for ev in display.poll_events() {
                match ev {
                    glutin::Event::Closed
                        => return,
                    glutin::Event::MouseMoved(x, y)
                        => self.mouse_moved(&display, x, y),
                    glutin::Event::MouseWheel(MouseScrollDelta::LineDelta(x, y), _)
                        => self.mouse_wheel_line(x, y),
                    glutin::Event::MouseInput(ElementState::Pressed, button)
                        => self.mouse_press(button),
                    glutin::Event::MouseInput(ElementState::Released, button)
                        => self.mouse_release(button),
                    _ => ()
                }
            }
            //// Update ////
            //// Render ////
            graphics.draw_polygon(&p);
            let window_size = display.get_window().unwrap().get_inner_size().unwrap();
            graphics.render(self.center, window_size.0, window_size.1, self.zoom);
            //// Sleep ////
            thread::sleep(time::Duration::from_millis(10));
        }
    }
    fn mouse_moved(&mut self, display: &GlutinFacade , x: i32, y: i32) {
        self.mouse_pos_past = self.mouse_pos;
        self.mouse_pos = Vec2::new(x as f64, y as f64);
        // Move the texture //
        if self.mouse_down {
            let window_size = display.get_window().unwrap().get_inner_size().unwrap();
            let mut offset = (self.mouse_pos - self.mouse_pos_past) / self.zoom;
            offset.x = -offset.x;
            offset.y =  offset.y;
            self.center += offset;
        }
    }

    fn mouse_wheel_line(&mut self, x: f32, y: f32) {
        // For each 'tick', it should *= factor
        const zoom_factor: f64 = 1.2;
        if y > 0.0 {
            self.zoom *= f64::powf(zoom_factor, y as f64);
        } else if y < 0.0 {
            self.zoom /= f64::powf(zoom_factor, -y as f64);
        }
    }

    fn mouse_press(&mut self, button: MouseButton) {
        self.mouse_down = true;
    }

    fn mouse_release(&mut self, button: MouseButton) {
        self.mouse_down = false;
    }

    fn new() -> Main {
        Main {
            zoom: 1.0,
            center: Vec2::new(0.0, 0.0),
            mouse_down: false,
            mouse_pos: Vec2::new(0.0, 0.0),
            mouse_pos_past: Vec2::new(0.0, 0.0),
        }
    }
}

fn make_polygon(num_edges: i32, inner_size: f64, outer_size: f64) -> Polygon {
    let mut vertices = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..num_edges {
        let angle = (i as f64 / num_edges as f64) * 2.0 * f64::consts::PI;
        let random_size = rng.gen::<f64>();
        let x = f64::cos(angle) * (inner_size + outer_size * random_size);
        let y = f64::sin(angle) * (inner_size + outer_size * random_size);
        vertices.push(Vec2::new(x, y));
    }
    Polygon::new(vertices)
}


