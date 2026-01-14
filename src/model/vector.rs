use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dist(&self, other: Self) -> f32 {
        (*self - other).length()
    }

    pub fn dist_to_line(&self, start: Self, end: Self) -> f32 {
        let length_sq = (end.x - start.x).powi(2) + (end.y - start.y).powi(2);
        if length_sq == 0.0 {
            return self.dist(start);
        }

        let t = ((self.x - start.x) * (end.x - start.x) + (self.y - start.y) * (end.y - start.y))
            / length_sq;
        let t = t.clamp(0.0, 1.0);

        let projection = Vector2::new(
            start.x + t * (end.x - start.x),
            start.y + t * (end.y - start.y),
        );

        self.dist(projection)
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
