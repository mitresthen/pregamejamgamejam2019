extern crate fs_extra;

fn main() {
    use fs_extra::dir;
    use std::path::Path;

    let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory

    use std::env;
    let out_dir = env::var("OUT_DIR").unwrap();

    let target_path = "./target/".to_owned() + out_dir.split("/target/").collect::<Vec<&str>>()[1].split("/build").collect::<Vec<&str>>()[0];
    let target_assets_path = "".to_owned() + &target_path + "/assets";

    if Path::new(&target_assets_path).exists() {
        let _ = std::fs::remove_dir_all(target_assets_path);
    }
    if std::fs::create_dir_all(&target_path).is_ok() {};
    dir::copy("./assets", target_path, &options).unwrap();
}
