use std::path::PathBuf;

pub fn get_test_resource_path(file: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test");
    d.push(file);
    
    d
}
