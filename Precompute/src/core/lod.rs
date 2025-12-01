use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LODLevel {
    pub level: u8,
    pub file: String,
    pub distance_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LODManifest {
    pub data: Vec<LODLevel>,
}

impl LODManifest {
    pub fn from_folder<P: AsRef<Path>>(folder: P) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_path = folder.as_ref().join("config.json");
        let file = File::open(&manifest_path)?;
        let reader = BufReader::new(file);
        let manifest: LODManifest = serde_json::from_reader(reader)?;

        for window in manifest.data.windows(2) {
            if window[0].level >= window[1].level {
                return Err("Los niveles LOD deben estar ordenados".into());
            }
            if window[0].distance_threshold >= window[1].distance_threshold {
                return Err("Los umbrales de distancia deben ser crecientes".into());
            }
        }

        Ok(manifest)
    }

    pub fn get_ply_paths<P: AsRef<Path>>(&self, folder: P) -> Vec<PathBuf> {
        self.data.iter()
            .map(|lod| folder.as_ref().join(&lod.file))
            .collect()
    }

    pub fn num_levels(&self) -> usize {
        self.data.len()
    }

    pub fn get_threshold(&self, level: u8) -> Option<f32> {
        self.data.iter()
            .find(|lod| lod.level == level)
            .map(|lod| lod.distance_threshold)
    }
}