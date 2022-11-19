use std::hash::Hash;
use std::path::Path;
use stb_image::image::LoadResult;
use stb_image::image;

use Error;


pub trait PixelType : Sized + PartialEq + Eq + Hash {
    fn from_bytes(it: &mut dyn Iterator<Item=u8>)
        -> Result<Option<Self>, Error>;

    fn channel_count() -> usize;
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl PixelType for RGBA {
    fn from_bytes(it: &mut dyn Iterator<Item=u8>)
        -> Result<Option<Self>, Error>
    {
        if let Some(r) = it.next() {
            let err = Error::IncompletePixel;

            let pixel =
                RGBA {
                    r,
                    g: it.next().ok_or_else(|| err.clone())?,
                    b: it.next().ok_or_else(|| err.clone())?,
                    a: it.next().ok_or_else(|| err.clone())?
                };

            Ok(Some(pixel))
        } else {
            Ok(None)
        }
    }

    fn channel_count() -> usize { 4 }
}

#[derive(Clone)]
pub struct Image<T: PixelType> {
    data: Vec<T>,
    width: i32,
    height: i32
}

impl<T: PixelType> Image<T> {
    pub fn load(filename: &str)
        -> Result<Image<T>, Error> {
        let png_img = match image::load(Path::new(filename)) {
            LoadResult::ImageU8(bytes) => { bytes },
            LoadResult::ImageF32(_) => panic!("Is float"),
            _ => return Err(Error::IO { path: Some(filename.to_string()) })
        };

        if png_img.depth != <T as PixelType>::channel_count() {
            return Err(Error::IncompatiblePixelType);
        }

        let mut byte_iterator = png_img.data.into_iter();

        let mut pixels : Vec<T> = Vec::new();

        while let Some(pixel) = T::from_bytes(&mut byte_iterator)? {
            pixels.push(pixel);
        }

        let image =
            Image {
                data: pixels,
                width: png_img.width as i32,
                height: png_img.height as i32
            };

        Ok(image)
    }

    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn data(&self) -> &[T] { &self.data }
}

#[test]
fn test_load_image() {
    let image : Image<RGBA> =
        Image::load("assets/stolen.png").unwrap();
}
