use std::fs::File;
use std::path::PathBuf;
use ply_rs::ply::{DefaultElement, Header, Ply};
use ply_rs::writer::Writer;
use crate::core::gaussian::Gaussian;

pub fn write_scene(path: &PathBuf, gaussians: &Vec<Gaussian>, header: Header) {
    let mut f = File::create(path).unwrap();

    let mut ply = Ply::<DefaultElement>::new();
    ply.header = header;

    let list: Vec<DefaultElement> = gaussians.iter().map(|g| (*g).into()).collect();
    ply.payload.insert("vertex".to_string(), list);

    let w = Writer::new();
    let written = w.write_ply(&mut f, &mut ply).unwrap();
    println!("Wrote {} bytes", written);
}