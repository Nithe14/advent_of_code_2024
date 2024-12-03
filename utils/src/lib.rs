use std::env;

fn default_path(file: &str) -> Option<String> {
    let project_root = env::var("CARGO_MANIFEST_DIR").ok()?;
    
    let path = format!("{project_root}/inputs/{file}");
    Some(path)
}

pub fn get_input_path(file: &str) -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            match default_path(file) {
                Some(f) => f,
                _ => {
                    eprintln!("Cannot determinate input file! Please use CARGO_MANIFEST_DIR or commandline argument.");
                    std::process::exit(1);
                }
            }
        })
} 
