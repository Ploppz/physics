use std::vec::Vec;


use glium;
use glium::{ Surface, Display };
use graphics;
use graphics::Color;
use graphics::renderer::BufferCollection;
use graphics::renderer::Renderer;
use geometry::vec::Vec2;
use geometry::polygon::Polygon;


pub struct LineRenderer<'a> {
    pub color: Color,
    display: &'a Display,
    // OpenGL state
    pos_col_prg: glium::Program,
    vertex_buf: glium::VertexBuffer<Vertex>,
    // Buffer management
    buffers: BufferCollection<Vertex>
}

const MAX_NUM_VERTICES: usize = 4000;


#[derive(Copy,Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

impl<'a> LineRenderer<'a> {
    pub fn new(display: &'a Display) -> LineRenderer<'a> {
        // OpenGL
        let pos_col_prg = graphics::create_program(display, "pos_col");
        let vertex_buffer = glium::VertexBuffer::empty(display, MAX_NUM_VERTICES).unwrap(); // empty vertex buffer
            // - check empty_dynamic, empty_persistent etc

        LineRenderer {
            display: display,
            color: Color { r: 1.0, b: 1.0, g: 1.0 },

            pos_col_prg: pos_col_prg,
            vertex_buf: vertex_buffer,

            buffers: BufferCollection::new(MAX_NUM_VERTICES)
        }
    }

    pub fn use_buffer(&mut self, name: String) {
        self.buffers.use_buffer(name);
    }
    pub fn clear_buffer(&mut self) {
        self.buffers.clear_buffer();
    }

    pub fn draw_line(&mut self, start: Vec2, end: Vec2) {
        self.buffers.push(Vertex {
            position: [start.x as f32, start.y as f32],
            color: [self.color.r, self.color.g, self.color.b],
        });
        self.buffers.push(Vertex {
            position: [end.x as f32, end.y as f32],
            color: [self.color.r, self.color.g, self.color.b],
        });
    }

}
impl<'a> Renderer for LineRenderer<'a> {
    fn render(&mut self, target: &mut glium::Frame, center: Vec2, width: u32, height: u32, zoom: f64) {
        let uniforms = uniform! {
            proj: graphics::proj_matrix(width as f64, height as f64, 0.0, 1.0),
            view: graphics::view_matrix(center.x, center.y, zoom, zoom),
            // we don't use them since there is no model transformation:
            center: [0.0 as f32, 0.0 as f32], 
            orientation: 0.0 as f32,
        };

        self.buffers.upload_all_vertices(&mut self.vertex_buf);
        let num_vertices = self.buffers.get_num_vertices();

        println!("num vertices = {}", num_vertices);
        
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        target.draw(self.vertex_buf.slice(0..num_vertices).unwrap(), indices, &self.pos_col_prg, &uniforms, &Default::default()).unwrap();

        self.buffers.clear_default_buffer();
    }

}

