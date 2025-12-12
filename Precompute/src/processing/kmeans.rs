use rand::rng;
use rand::prelude::IndexedRandom;
use nalgebra::Vector3;

pub struct KMeans {
    k: usize,
    max_iterations: usize,
    tolerance: f32,
}

impl KMeans {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iterations: 100,
            tolerance: 1e-4,
        }
    }

    pub fn compute_centroids(&self, points: &[Vector3<f32>]) -> Option<Vec<Vector3<f32>>> {
        if points.is_empty() || points.len() < self.k {
            return None;
        }

        let mut rng = rng();
        let mut centroids: Vec<Vector3<f32>> = points
            .choose_multiple(&mut rng, self.k)
            .copied()
            .collect();

        for _iteration in 0..self.max_iterations {

            let assignments = self.assign_to_nearest_centroid(points, &centroids);

            let new_centroids = self.compute_new_centroids(points, &assignments);

            if self.has_converged(&centroids, &new_centroids) {
                return Some(new_centroids);
            }

            centroids = new_centroids;
        }

        Some(centroids)
    }

    fn assign_to_nearest_centroid(
        &self,
        points: &[Vector3<f32>],
        centroids: &[Vector3<f32>],
    ) -> Vec<usize> {
        points
            .iter()
            .map(|point| {
                centroids
                    .iter()
                    .enumerate()
                    .map(|(idx, centroid)| (idx, (point - centroid).norm()))
                    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .map(|(idx, _)| idx)
                    .unwrap()
            })
            .collect()
    }

    fn compute_new_centroids(
        &self,
        points: &[Vector3<f32>],
        assignments: &[usize],
    ) -> Vec<Vector3<f32>> {
        let mut new_centroids = vec![Vector3::zeros(); self.k];
        let mut counts = vec![0usize; self.k];

        for (point, &cluster_idx) in points.iter().zip(assignments.iter()) {
            new_centroids[cluster_idx] += point;
            counts[cluster_idx] += 1;
        }

        for (centroid, count) in new_centroids.iter_mut().zip(counts.iter()) {
            if *count > 0 {
                *centroid /= *count as f32;
            }
        }

        new_centroids
    }

    fn has_converged(
        &self,
        old_centroids: &[Vector3<f32>],
        new_centroids: &[Vector3<f32>],
    ) -> bool {
        old_centroids
            .iter()
            .zip(new_centroids.iter())
            .all(|(old, new)| (old - new).norm() < self.tolerance)
    }
}