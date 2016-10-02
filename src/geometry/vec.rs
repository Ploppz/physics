// All Vec2tors are of 2 dimentions

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
    }
    pub fn dot(a: Vec2, b: Vec2) -> f64 {
        a.x * b.x + a.y * b.y
    }
    pub fn cross(a: Vec2, b: Vec2) -> f64 {
        a.x * b.y - a.y * b.x
    }
}

//// Operators Vec2 & Vec2 ////
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}
impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x * other.x, y: self.y * other.y }
    }
}
impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x / other.x, y: self.y / other.y }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Vec2) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Vec2) {
        self.x /= other.x;
        self.y /= other.y;
    }
}



//// Operators Vec2 & float ////
impl Add<f64> for Vec2 {
    type Output = Vec2;
    fn add(self, n: f64) -> Vec2 {
        Vec2 { x: self.x + n, y: self.y + n }
    }
}
impl Sub<f64> for Vec2 {
    type Output = Vec2;
    fn sub(self, n: f64) -> Vec2 {
        Vec2 { x: self.x - n, y: self.y - n }
    }
}
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, n: f64) -> Vec2 {
        Vec2 { x: self.x * n, y: self.y * n }
    }
}
impl Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, n: f64) -> Vec2{
        Vec2 { x: self.x / n, y: self.y / n }
    }
}
