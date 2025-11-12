use std::path::{Path, PathBuf};
use crate::core::lod::ConfigLOD;

pub struct Scene {
    lod_levels: Vec<ConfigLOD>,
    origin_file: PathBuf,
}
impl Scene {
    pub fn new(origin_file: &String) -> Scene {
        Scene{
            lod_levels: Vec::new(),
            origin_file: PathBuf::from(origin_file),
        }
    }
    pub fn add_lod(&mut self, lod_level: ConfigLOD) {
        self.lod_levels.push(lod_level);
    }
}