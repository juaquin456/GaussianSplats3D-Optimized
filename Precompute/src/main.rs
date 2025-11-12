use std::path::PathBuf;

mod core;
mod io;
mod processing;

fn main() {
    // TODO: get from args command line
    const L: i16 = 8; // LODs
    let path = "<>.ply".to_string();

    let mut scene = core::scene::Scene::new(&path);
    scene.add_lod(core::lod::ConfigLOD::new(0i8, 0f32, &path));

    // read
    let l0 = io::scene_loader::read_scene(path);

    // build LODs
    for i in 1..L+1 {
        let new_scene = processing::lod_builder::build_next_lod(&l0, i as f32);
        // TODO: Get a right threshold distance
        // new_scene.save()
    }

    // calculate a subdivision
}
