// All vectors are of 2 dimentions

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Default)]
pub struct vec {
    pub x: f64,
    pub y: f64,
}

impl vec {
    pub fn new(x: f64, y: f64) -> vec {
        vec { x: x, y: y }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
    }
    pub fn dot(a: vec, b: vec) -> f64 {
        a.x * b.x + a.y * b.y
    }
    pub fn cross(a: vec, b: vec) -> f64 {
        a.x * b.y - a.y * b.x
    }
}

//// Operators vec & vec ////
impl Add for vec {
    type Output = vec;
    fn add(self, other: vec) -> vec {
        vec { x: self.x + other.x, y: self.y + other.y }
    }
}
impl Sub for vec {
    type Output = vec;
    fn sub(self, other: vec) -> vec {
        vec { x: self.x - other.x, y: self.y - other.y }
    }
}
impl Mul for vec {
    type Output = vec;
    fn mul(self, other: vec) -> vec {
        vec { x: self.x * other.x, y: self.y * other.y }
    }
}
impl Div for vec {
    type Output = vec;
    fn div(self, other: vec) -> vec {
        vec { x: self.x / other.x, y: self.y / other.y }
    }
}
impl AddAssign for vec {
    fn add_assign(&mut self, other: vec) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for vec {
    fn sub_assign(&mut self, other: vec) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl MulAssign for vec {
    fn mul_assign(&mut self, other: vec) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
impl DivAssign for vec {
    fn div_assign(&mut self, other: vec) {
        self.x /= other.x;
        self.y /= other.y;
    }
}



//// Operators vec & float ////
impl Add<f64> for vec {
    type Output = vec;
    fn add(self, n: f64) -> vec {
        vec { x: self.x + n, y: self.y + n }
    }
}
impl Sub<f64> for vec {
    type Output = vec;
    fn sub(self, n: f64) -> vec {
        vec { x: self.x - n, y: self.y - n }
    }
}
impl Mul<f64> for vec {
    type Output = vec;
    fn mul(self, n: f64) -> vec {
        vec { x: self.x * n, y: self.y * n }
    }
}
impl Div<f64> for vec {
    type Output = vec;
    fn div(self, n: f64) -> vec{
        vec { x: self.x / n, y: self.y / n }
    }
}
