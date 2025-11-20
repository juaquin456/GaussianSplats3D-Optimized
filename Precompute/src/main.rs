use std::path::PathBuf;

mod core;
mod io;
mod processing;

fn main() {
    // TODO: get from args command line
    let path = "<>.ply".to_string();
    let mode = "KMEANS";

    let mut output_dir = PathBuf::from(&path);
    output_dir.pop();

    let mut scene = core::scene::Scene::new(path);

    // calculate a subdivision
}
