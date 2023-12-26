use std::f64;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn distance(&self, other: &Vector2D) -> f64 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        (dx * dx + dy * dy).sqrt()
    }
    pub fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x() + self.y * other.y()
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
impl Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x + other,
            y: self.y + other,
        }
    }
}
impl Div<f64> for Vector2D {
    type Output = Vector2D;
    fn div(self, other: f64) -> Vector2D {
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
