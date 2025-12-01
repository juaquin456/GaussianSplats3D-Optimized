use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox2D {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl BoundingBox2D {
    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        Self { min, max }
    }

    pub fn from_points(points: &[Vector2<f32>]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        let mut min = points[0];
        let mut max = points[0];

        for point in points.iter().skip(1) {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
        }

        Some(Self { min, max })
    }

    pub fn contains(&self, point: &Vector2<f32>) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn center(&self) -> Vector2<f32> {
        (self.min + self.max) / 2.0
    }
}
