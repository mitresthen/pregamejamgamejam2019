// cargo-deps: fs_extra = "1.1.0", walkdir = "2.3.1", zip = "0.5.4"
#![allow(warnings)]
extern crate fs_extra;
extern crate walkdir;
extern crate zip;

use std::io::prelude::*;
use std::io::{Write, Seek};
use std::iter::Iterator;
use std::path::Path;

use std::fs::File;
use walkdir::{WalkDir, DirEntry};

fn main() {
    if Path::new("./target/release/game").exists() {
        std::fs::remove_dir_all("./target/release/game");
    }
    std::fs::create_dir_all("./target/release/game");

    let options = fs_extra::dir::CopyOptions::new(); //Initialize default values for CopyOptions
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
    fs_extra::dir::copy("./target/release/assets", "./target/release/game", &options);

    let options = fs_extra::file::CopyOptions::new(); //Initialize default values for CopyOptions
    let exe_name = find_exe_name(Path::new("./target/release")).unwrap();
    fs_extra::file::copy("./target/release/".to_string() + &exe_name, "./target/release/game/".to_string() + &exe_name, &options);
    fs_extra::file::copy("./target/release/sdl2.dll", "./target/release/game/sdl2.dll", &options);

    let src_dir = "./target/release/game";
    let dst_file = "./target/release/game.zip";
    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2].iter() {
        if method.is_none() { continue }
        match doit(src_dir, dst_file, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", src_dir, dst_file),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

fn find_exe_name(dir: &Path) -> Result<String, std::io::Error> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            } else {
                let file_name = entry.file_name().into_string().unwrap();
                if file_name.ends_with(".exe")
                {
                    return Ok(file_name);
                }
            }
        }
    }
    return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
}

const METHOD_STORED : Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);

#[cfg(feature = "deflate")]
const METHOD_DEFLATED : Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
#[cfg(not(feature = "deflate"))]
const METHOD_DEFLATED : Option<zip::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2 : Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2 : Option<zip::CompressionMethod> = None;

fn zip_dir<T>(it: &mut dyn Iterator<Item=DirEntry>, prefix: &str, writer: T, method: zip::CompressionMethod)
              -> zip::result::ZipResult<()>
    where T: Write+Seek
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = zip::write::FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(src_dir: &str, dst_file: &str, method: zip::CompressionMethod) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(zip::result::ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}
