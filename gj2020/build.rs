#![allow(warnings)]
extern crate fs_extra;

fn main() {
    use fs_extra::dir;
    use std::path::Path;

    let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory

    if Path::new("./target/debug/assets").exists() {
        std::fs::remove_dir_all("./target/debug/assets");
    }
    if std::fs::create_dir_all("./target/debug").is_ok() { };
    dir::copy("./assets", "./target/debug", &options).unwrap();

    if Path::new("./target/release/assets").exists() {
        std::fs::remove_dir_all("./target/release/assets");
    }
    if std::fs::create_dir_all("./target/release").is_ok() { };
    dir::copy("./assets", "./target/release", &options).unwrap();
}
