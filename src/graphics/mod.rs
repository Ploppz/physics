use std::fs::File;
use std::io::Read;

use glium;
use glium::{ Surface, Display };

use geometry::polygon::Polygon;
use geometry::vec::vec;
use graphics::renderer::Renderer;
use graphics::renderer::line::LineRenderer;

pub mod renderer;

pub struct Graphics<'a> {
    display: &'a Display,
    lines: LineRenderer<'a>,
}

impl<'a> Graphics<'a> {
    pub fn new(display: &'a Display) -> Graphics {
        Graphics {
            display: display,
            lines: LineRenderer::new(display),
        }
    }
    /// Renders all buffers, clears all 'default' buffers
    pub fn render(&mut self, center: vec, width: u32, height: u32, zoom: f64) {
        let mut target = self.display.draw();        // target: glium::Frame
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        self.lines.render(&mut target, center, width, height, zoom);

        target.finish().unwrap(); 
    }

    //// Draw commands ////
    pub fn draw_polygon(&mut self, p: &Polygon) {
        for edge in p.world_edges() {
            self.lines.draw_line(edge.get_start(), edge.get_end());
        }
    }

    //// Buffer management ////
    fn use_buffer(&mut self, name: String) {
        self.lines.use_buffer(name);
        // ADD AS YOU GO
    }
    fn clear_buffer(&mut self) {
        self.lines.clear_buffer();
        // ADD AS YOU GO
    }
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0 }
    }
    pub fn set(&mut self, r: f32, g: f32, b: f32) {
        self.r = r; self.g = g; self.b = b;
    }
}


//// Helpers ////
pub fn create_program<'a, F>(display: &'a F, name: &'static str) -> glium::Program
    where F: glium::backend::Facade
{
    let mut f = File::open("shaders/".to_string() + name + ".vert").unwrap();
    let mut vert_src = String::new();
    f.read_to_string(&mut vert_src);
    f = File::open("shaders/".to_string() + name + ".frag").unwrap();
    let mut frag_src = String::new();
    f.read_to_string(&mut frag_src);

    glium::Program::from_source(display, vert_src.as_str(), frag_src.as_str(), None).unwrap()
}

pub fn view_matrix(centerX: f64, centerY: f64, scaleX: f64, scaleY: f64) -> [[f32; 4]; 4] {
    // data views the transpose of the actual matrix
    let scaleX = scaleX as f32;
    let scaleY = scaleY as f32;
    let centerX = centerX as f32;
    let centerY = centerY as f32;
    [
        [ scaleX,	0.0, 0.0,	0.0 ],
        [ 	0.0,	scaleY, 0.0,	0.0 ],
        [ 	0.0,	0.0,	1.0,	0.0 ],
        [ -centerX * scaleX,	-centerY * scaleY,	0.0,	1.0]
    ]
}
pub fn proj_matrix(width: f64, height: f64, far: f64, near: f64) -> [[f32; 4]; 4] {
    let width = width as f32;
    let height = height as f32;
    let far = far as f32;
    let near = near as f32;
    [
        [2.0/width, 0.0, 			0.0, 							0.0],
        [0.0, 			 2.0/height,  0.0, 							0.0],
        [0.0, 			 0.0,  			-2.0/(far - near), 			0.0],
        [0.0, 			 0.0, 			-(far + near)/(far - near), 1.0]
    ]
}
