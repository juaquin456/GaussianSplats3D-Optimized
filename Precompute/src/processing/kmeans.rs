use crate::processing::{PartitionStrategy, Point3};
use rand::Rng;

pub struct KMeans {
    k: usize,
    max_iters: usize,
}

impl KMeans {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iters: 100
        }
    }

    pub fn suggest_k(points: &[Point3], d1: f32) -> usize {
        if points.is_empty() { return 1; }

        let n = points.len() as f32;
        let sum = points.iter().fold([0.0, 0.0, 0.0], |acc, p| {
            [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]]
        });
        let center_avg = [sum[0] / n, sum[1] / n, sum[2] / n];

        let max_dist = points.iter()
            .map(|p| dist_euclidean(&center_avg, p))
            .fold(0.0f32, f32::max);

        if d1 <= 0.001 { return 1; }

        let k_float = (4.0 / d1) * max_dist;

        let k = k_float.ceil() as usize;
        std::cmp::max(1, k)
    }
}



impl PartitionStrategy for KMeans {
    fn calculate_centers(&self, points: &[Point3]) -> Vec<Point3> {
        let n_points = points.len();

        if n_points == 0 || self.k == 0 {
            return vec![];
        }
        if self.k >= n_points {
            return points.to_vec();
        }

        let mut rng = rand::thread_rng();
        let mut centroids: Vec<Point3> = (0..self.k)
            .map(|_| points[rng.gen_range(0..n_points)])
            .collect();

        for i in 0..self.max_iters {
            let mut sums = vec![(0.0, 0.0, 0.0); self.k];
            let mut counts = vec![0; self.k];
            let mut changed = false;

            for p in points {
                let cluster_idx = find_nearest_centroid(p, &centroids);

                sums[cluster_idx].0 += p[0];
                sums[cluster_idx].1 += p[1];
                sums[cluster_idx].2 += p[2];
                counts[cluster_idx] += 1;
            }

            for k in 0..self.k {
                if counts[k] == 0 { continue; }

                let count_f32 = counts[k] as f32;
                let new_centroid = [
                    sums[k].0 / count_f32,
                    sums[k].1 / count_f32,
                    sums[k].2 / count_f32,
                ];

                if dist_sq(&centroids[k], &new_centroid) > 0.0001 {
                    centroids[k] = new_centroid;
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }
        centroids
    }
}

fn find_nearest_centroid(p: &Point3, centroids: &[Point3]) -> usize {
    let mut min_dist = f32::MAX;
    let mut best_idx = 0;

    for (i, c) in centroids.iter().enumerate() {
        let d = dist_sq(p, c);
        if d < min_dist {
            min_dist = d;
            best_idx = i;
        }
    }
    best_idx
}

fn dist_sq(a: &Point3, b: &Point3) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx*dx + dy*dy + dz*dz
}

fn dist_euclidean(a: &Point3, b: &Point3) -> f32 {
    dist_sq(a, b).sqrt()
}