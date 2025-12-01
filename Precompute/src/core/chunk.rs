use nalgebra::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: usize,
    pub filename: String,
    pub centroid: Vector3<f32>,
    pub xy_polygon: Vec<Vector2<f32>>,
    pub neighbors: Vec<(usize, usize)>,
}

impl Chunk {
    pub fn new(
        id: usize,
        filename: String,
        centroid: Vector3<f32>,
        xy_polygon: Vec<Vector2<f32>>,
        neighbors: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            id,
            filename,
            centroid,
            xy_polygon,
            neighbors,
        }
    }

    pub fn radius(&self) -> f32 {
        let centroid_2d = Vector2::new(self.centroid.x, self.centroid.y);

        self.xy_polygon
            .iter()
            .map(|vertex| (vertex - centroid_2d).norm())
            .fold(0.0f32, f32::max)
    }
}