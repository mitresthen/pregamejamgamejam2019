extern crate fs_extra;

fn main() {
    use fs_extra::dir;

    let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory

    dir::copy("./assets", "./target/debug", &options);
    dir::copy("./assets", "./target/release", &options);
}
