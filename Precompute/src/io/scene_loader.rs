use ply_rs::ply::{DefaultElement, Header};
use ply_rs::parser;
use std::fs::File;
use std::path::Path;
use crate::core::gaussian::Gaussian;

pub fn read_scene<P: AsRef<Path>>(path: P) -> Result<(Vec<Gaussian>, Header), Box<dyn std::error::Error>> {
    let mut f = File::open(path.as_ref())?;

    let parser = parser::Parser::<DefaultElement>::new();
    let ply = parser.read_ply(&mut f)
        .map_err(|e| format!("Error al parsear PLY: {:?}", e))?;

    let vertices = ply.payload.get("vertex")
        .ok_or("No se encontró el elemento 'vertex' en el PLY")?;

    let gaussians: Vec<Gaussian> = vertices.iter().map(Gaussian::from).collect();

    Ok((gaussians, ply.header))
}