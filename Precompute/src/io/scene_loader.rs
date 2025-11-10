use ply_rs::ply::DefaultElement;
use ply_rs::parser;
use std::fs::File;
use crate::data::gaussian::Gaussian;

pub fn read_scene(path: String) -> Vec<Gaussian> {
    let mut f = File::open(path).unwrap();

    let parser = parser::Parser::<DefaultElement>::new();
    let ply = parser.read_ply(&mut f).unwrap();

    let vertices = ply.payload.get("vertex").unwrap();

    vertices.iter().map(Gaussian::from).collect()
}