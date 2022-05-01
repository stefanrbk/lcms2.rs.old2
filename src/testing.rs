use std::{env::temp_dir, path::PathBuf};

use chrono::{Datelike, Timelike};

pub fn get_test_resource_path(file: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test");
    d.push(file);

    d
}

pub fn get_temp_file_path(test_name: &str) -> PathBuf {
    let mut temp = temp_dir();
    let now = chrono::Utc::now();
    let file = format!(
        "lcms2.rs_{}_{}-{}-{}-{}-{}-{}-{}.dat",
        test_name,
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.timestamp_subsec_millis()
    );

    temp.push(file);

    temp
}
