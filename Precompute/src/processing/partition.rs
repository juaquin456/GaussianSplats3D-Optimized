use nalgebra::Vector3;
use crate::core::chunk::Chunk;
use crate::core::bounding_box::BoundingBox2D;

pub struct PartitionInput<'a> {
    pub camera_positions: &'a [Vector3<f32>],
    pub bounding_box: &'a BoundingBox2D,
}

pub trait PartitionStrategy {
    fn compute_chunks(
        &self,
        input: &PartitionInput,
    ) -> Result<Vec<Chunk>, Box<dyn std::error::Error>>;
}