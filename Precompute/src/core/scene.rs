use std::path::{Path, PathBuf};
use ply_rs::ply::Header;
use crate::io::colmap_loader::{self, CameraPose};
use crate::io::scene_loader;
use crate::core::gaussian::Gaussian;
use crate::processing::{PartitionStrategy, Point3};

pub struct Scene {
    origin_file: PathBuf,
    pub cameras: Vec<CameraPose>,
    pub gaussians: Option<Vec<Gaussian>>,
    pub header: Option<Header>,
}
impl Scene {
    pub fn new<P: AsRef<Path>>(origin_file: P, cameras_file: P) -> Scene {
        let cameras = colmap_loader::read_colmap_images(cameras_file);

        Scene{
            origin_file: origin_file.as_ref().to_path_buf(),
            cameras,
            gaussians: None,
            header: None,
        }
    }

    pub fn load_data(&mut self) {
        if self.gaussians.is_some() {
            println!("Gaussian is already loaded");
            return;
        }

        let (data, header) = scene_loader::read_scene(&self.origin_file);

        println!("{} gaussians loaded", data.len());
        self.gaussians = Some(data);
        self.header = Some(header);
    }

    pub fn calculate_chunks(&self, strategy: &dyn PartitionStrategy) -> Vec<Point3> {
        let points: Vec<Point3> = self.cameras.iter().map(|c| c.center).collect();
        strategy.calculate_centers(&points)
    }
}
