mod data;
mod io;
fn main() {
    // TODO: get from args command line
    let path = "<>.ply".to_string();

    // read
    let scene = io::scene_loader::read_scene(path);

    // calculate a subdivision

    // build LODs
}
