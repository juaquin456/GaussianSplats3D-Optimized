pub type Point3 = [f32; 3];

pub trait PartitionStrategy {
    fn calculate_centers(&self, camera_positions: &[Point3]) -> Vec<Point3>;
}

pub mod kmeans;