use std::path::{Path, PathBuf};
pub struct ConfigLOD {
    l: i8,
    d_l: f32,
    file: PathBuf,
}
impl ConfigLOD {
    pub fn new(l: i8, d_l: f32, file: &String) -> ConfigLOD {
        ConfigLOD { l, d_l, file: PathBuf::from(file) }
    } 
}