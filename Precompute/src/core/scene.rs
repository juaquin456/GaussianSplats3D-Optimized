use std::path::{Path, PathBuf};
use crate::core::lod::ConfigLOD;

pub struct Scene {
    origin_file: PathBuf,
}
impl Scene {
    pub fn new<P: AsRef<Path>>(origin_file: P) -> Scene {
        Scene{
            origin_file: origin_file.as_ref().to_path_buf(),
        }
    }

    /*
    pub fn calculate_subdivision(enum mode) {
        // read ply
        Config // geometria de cada chunk, num chunks, filename chunk
        switch {
            KMeans::Execute(ply)
            Regular::Square
        }

    }
     */
}