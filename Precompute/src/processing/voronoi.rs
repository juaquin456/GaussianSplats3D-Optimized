use nalgebra::{Vector2, Vector3};
use voronator::VoronoiDiagram as VoronatorDiagram;
use voronator::delaunator::Point;
use crate::core::bounding_box::BoundingBox2D;

pub struct VoronoiDiagram;

impl VoronoiDiagram {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_cells(
        &self,
        centroids: &[Vector3<f32>],
        bbox: &BoundingBox2D,
    ) -> Result<Vec<Vec<Vector2<f32>>>, Box<dyn std::error::Error>> {
        if centroids.is_empty() {
            return Err("Cannot compute Voronoi cells without centroids".into());
        }

        let points: Vec<(f64, f64)> = centroids
            .iter()
            .map(|c| (c.x as f64, c.y as f64))
            .collect();

        let min_corner = (bbox.min.x as f64, bbox.min.y as f64);
        let max_corner = (bbox.max.x as f64, bbox.max.y as f64);

        let diagram = VoronatorDiagram::<Point>::from_tuple(
            &min_corner,
            &max_corner,
            &points
        ).ok_or_else(|| {
            "Failed to compute Voronoi diagram".to_string()
        })?;

        let mut polygons = Vec::new();

        for cell in diagram.cells() {
            let vertices: Vec<Vector2<f32>> = cell.points()
                .into_iter()
                .map(|p| Vector2::new(p.x as f32, p.y as f32))
                .collect();

            if vertices.len() >= 3 {
                polygons.push(vertices);
            } else {
                eprintln!("Warning: Cell with fewer than 3 vertices ignored");
            }
        }

        if polygons.len() != centroids.len() {
            return Err(format!(
                "Mismatch: {} centroids generated {} polygons",
                centroids.len(),
                polygons.len()
            ).into());
        }

        Ok(polygons)
    }
}