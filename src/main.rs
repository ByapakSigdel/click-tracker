use std::path::PathBuf;

fn main() {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("log4rs.yaml");
    
    log4rs::init_file(config_path, Default::default()).unwrap();

    key_mouse_tracker::start_tracking();
}
