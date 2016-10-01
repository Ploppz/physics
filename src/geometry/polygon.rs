use std::vec::Vec;
use core::iter::Iterator;
use geometry::vec::vec;

pub struct Polygon {
    vertices: Vec<vec>,

    translation: vec,
    orientation: f64,
}

impl Polygon {
    pub fn new(vertices: Vec<vec>) -> Polygon {
        Polygon {
            vertices: vertices,
            translation: vec::new(0.0, 0.0),
            orientation: 0.0,
        }
    }

    pub fn num_vertices(&self) -> i32 {
        self.vertices.len() as i32
    }

    pub fn model_vertex(&self, index: i32) -> vec {
        let new_index = ((index + self.num_vertices()) % self.num_vertices()) as usize;
        self.vertices[new_index]
    }

    pub fn world_vertex(&self, index: i32) -> vec {
        self.transform(self.model_vertex(index))
    }

    pub fn transform(&self, model_vec: vec) -> vec {
        let c = self.orientation.cos();
        let s = self.orientation.sin();
        vec {
            x:   c*model_vec.x + s*model_vec.y + self.translation.x,
            y: - s*model_vec.x + c*model_vec.y + self.translation.y,
        }
    }

    pub fn detransform(&self, mut world_vec: vec) -> vec {
        world_vec -= self.translation;
        let c = self.orientation.cos();
        let s = self.orientation.sin();
        vec {
            x:   c*world_vec.x + s*world_vec.y,
            y: - s*world_vec.x + c*world_vec.y,
        }
    }

    pub fn model_edges(&self) -> EdgeIterator {
        EdgeIterator::new(self, false)
    }
    pub fn world_edges(&self) -> EdgeIterator {
        EdgeIterator::new(self, true)
    }

}


//// Iteration ////

pub struct EdgeIterator<'a> {
    polygon: &'a Polygon,
    index: i32,                 // index of the *end* of the edge
    start: vec,
    end: vec,
    transformed: bool,          // Temporary solution
}


impl<'a> EdgeIterator<'a> {
    pub fn new(polygon: &'a Polygon, transformed: bool) -> EdgeIterator {
        println!("Start edge iter");
        let start = match transformed {
            true => polygon.world_vertex(-1),
            false => polygon.model_vertex(-1),
        };
        let end = match transformed {
            true => polygon.world_vertex(0),
            false => polygon.model_vertex(0),
        };

        EdgeIterator {
            polygon: polygon,
            index: 0,
            start: start,
            end: end,
            transformed: transformed,
        }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;
    fn next(&mut self) -> Option<Edge> {
        if self.index == self.polygon.num_vertices() {
            return None;
        }
        self.index += 1;
        println!("Edge iter index {}", self.index);
        self.start = self.end;
        if self.transformed {
            self.end = self.polygon.world_vertex(self.index);
        } else {
            self.end = self.polygon.model_vertex(self.index);
        }
        Some(Edge {
            index: self.index,
            start: self.start,
            end:self.end,
        })
    }
}

pub struct Edge { // almost the same as EdgeIterator
    index: i32,
    start: vec,
    end: vec,
}
impl Edge {
    pub fn get_index(&self) -> i32 { self.index }
    pub fn get_start(&self) -> vec { self.start}
    pub fn get_end(&self) -> vec { self.end}
}
