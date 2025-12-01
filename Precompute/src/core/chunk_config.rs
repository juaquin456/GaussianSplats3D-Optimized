use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use crate::processing::Point3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub id: usize,
    pub filename: String,
    pub centroid: Point3,
    pub xy_polygon: Vec<Point2>,
    pub neighs: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkConfig {
    pub folder: String,
    pub chunks: Vec<ChunkMetadata>,
}

impl ChunkConfig {
    pub fn new(folder: String) -> Self {
        ChunkConfig {
            folder,
            chunks: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, metadata: ChunkMetadata) {
        self.chunks.push(metadata);
    }

    pub fn calculate_neighbors(&mut self, threshold_multiplier: f32) {
        let n = self.chunks.len();

        let avg_radius = if n > 0 {
            let total_radius: f32 = self.chunks.iter()
                .map(|c| {
                    c.xy_polygon.iter()
                        .map(|p| {
                            let dx = p.x - c.centroid[0];
                            let dy = p.y - c.centroid[1];
                            (dx*dx + dy*dy).sqrt()
                        })
                        .sum::<f32>() / c.xy_polygon.len() as f32
                })
                .sum();
            total_radius / n as f32
        } else {
            10.0
        };

        let neighbor_threshold = avg_radius * threshold_multiplier;

        for i in 0..n {
            let mut neighbors = Vec::new();
            let center_i = self.chunks[i].centroid;

            for j in 0..n {
                if i == j { continue; }

                let center_j = self.chunks[j].centroid;

                let dx = center_i[0] - center_j[0];
                let dy = center_i[1] - center_j[1];
                let dist = (dx*dx + dy*dy).sqrt();

                if dist < neighbor_threshold {
                    neighbors.push((j, 0));
                }
            }

            println!("  Chunk #{}: {} neighbors", i, neighbors.len());
            self.chunks[i].neighs = neighbors;
        }
    }

    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

pub fn generate_approximate_polygon(cx: f32, cy: f32, radius: f32, num_vertices: usize) -> Vec<Point2> {
    let mut polygon = Vec::new();

    for i in 0..num_vertices {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (num_vertices as f32);
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin();
        polygon.push(Point2 { x, y });
    }

    polygon
}