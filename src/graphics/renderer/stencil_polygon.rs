
use graphics::renderer::BufferCollection;
use graphics::renderer::Renderer;
use geometry::vec::vec;
use geometry::polygon::Polygon;


pub struct StencilPolygonRenderer<'a> {
    display: &'a Display,
    color: Color,
    // OpenGL state
    const_color_prg: glium::Program,
    pos_prg: glium::Program,
    vertex_buf: glium::VertexBuffer<Vertex>,
    // Buffer management
    buffers: BufferCollection<Vertex>
}

const MAX_NUM_VERTICES: usize = 4000;

#[derive(Copy,Clone)]
struct Vertex {
    position: [f32; 2],
}

impl<'a> StencilPolygonRenderer<'a> {
    pub fn new(display: &'a Display) -> StencilPolygonRenderer<'a> {
        let const_color_prg = graphics::create_program(display, "const_color");
        let pos_prg = graphics::create_program(display, "pos");
        let vertex_buffer = glium::VertexBuffer::empty(display, MAX_NUM_VERTICES).unwrap(); // empty vertex buffer

        StencilPolygonRenderer {
            display: display,
            color: Color { r: 1.0, b: 1.0, g: 1.0 },

            const_color_prg: const_color_prg,
            pos_prg: pos_prg,
            vertex_buf: vertex_buffer,

            buffers: BufferCollection::new(MAX_NUM_VERTICES)
        }
    }
    pub fn use_buffer(&mut self, name: String) {
        self.buffers.use_buffer(name);
    }
    pub fn clear_buffer(&mut self) {
        self.triangle_fans.clear();
        self.buffers.clear_buffer();
    }

    pub fn draw_polygon(&mut self, polygon: Polygon) {
        // TODO make iterator for vertices
        
        let t = TriangleFan {
            start_elment_index = 
        };

        for edge in polygon.model_edges() {
            self.buffers.push_back(Vertex {
                position: [edge.start.x as f64, edge.start.y as f64],
            })
        }
    }
}
