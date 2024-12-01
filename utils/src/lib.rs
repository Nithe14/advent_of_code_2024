use std::env;

pub fn get_input_path(file: &str) -> String {
    let project_root = env::var("CARGO_MANIFEST_DIR")
        .expect("Cannot get project root path!");
    
    let path = format!("{project_root}/inputs/{file}");
    path
}
