use std::f32;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn distance(&self, other: &Vector2D) -> f32 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        (dx * dx + dy * dy).sqrt()
    }
    pub fn ortho(&self) -> Vector2D {
        Vector2D::new(-self.y, self.x)
    }
    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x() + self.y * other.y()
    }
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn mag_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn normalized(&self) -> Vector2D {
        let d = self.magnitude();
        Vector2D {
            x: self.x / d,
            y: self.y / d,
        }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }
}
impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x(),
            y: self.y - other.y(),
        }
    }
}
impl Neg for Vector2D {
    type Output = Vector2D;
    fn neg(self) -> Vector2D {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl Mul<f32> for Vector2D {
    type Output = Vector2D;
    fn mul(self, other: f32) -> Vector2D {
        Vector2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl Div<f32> for Vector2D {
    type Output = Vector2D;
    fn div(self, other: f32) -> Vector2D {
        Vector2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
