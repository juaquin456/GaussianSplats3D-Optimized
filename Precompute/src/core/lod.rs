use std::path::{Path, PathBuf};
pub struct ConfigLOD {
    level: u8,
    distance: f32,
    file: PathBuf,
}
impl ConfigLOD {
    pub fn new<P: AsRef<Path>>(level: u8, distance: f32, file: P) -> ConfigLOD {
        ConfigLOD { level, distance, file: file.as_ref().to_path_buf() }
    }
}