use std::path::Path;
use stb_image::image::LoadResult;
use stb_image::image;

use std::collections::BTreeMap;

use super::Error;

mod sdl {
pub use sdl2::render::TextureCreator;
pub use sdl2::render::Texture;
pub use sdl2::video::WindowContext;
pub use sdl2::surface::Surface;
pub use sdl2::pixels::PixelFormatEnum;
}

pub struct Texture {
    index: usize,
    width: i32,
    height: i32
}

impl Texture {
    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
}

pub struct TextureData<'t> {
    //surface: sdl::Surface<'static>,
    texture: sdl::Texture<'t>
}

pub struct TextureRegistry<'t> {
    textures: BTreeMap<usize, TextureData<'t>>,
    texture_creator: &'t sdl::TextureCreator<sdl::WindowContext>
}

impl<'t> TextureRegistry<'t> {
    pub fn new(texture_creator: &'t sdl::TextureCreator<sdl::WindowContext>) -> TextureRegistry<'t> {
        TextureRegistry {
            textures: BTreeMap::new(),
            texture_creator: texture_creator
        }
    }
    pub fn load(&mut self, path: &str) -> Result<Texture, Error> {
        let png_img = match image::load(Path::new(path)) {
            LoadResult::ImageU8(bytes) => { bytes },
            LoadResult::ImageF32(_) => panic!("Is float"),
            _ => return Err(Error::IO { path: Some(path.to_string()) })
        };

        let mut surface =
            sdl::Surface::new(
                png_img.width as u32,
                png_img.height as u32,
                match png_img.depth {
                    3 => sdl::PixelFormatEnum::BGR888,
                    4 => sdl::PixelFormatEnum::ABGR8888,
                    _ => return Err(Error::UnsupportedPixelFormat)
                }
            ).unwrap();

        surface.with_lock_mut(
            |buffer| {
                for (dst, src) in buffer.iter_mut().zip(png_img.data.iter()) {
                    *dst = *src
                }
            }
        );

        let texture = self.texture_creator
                .create_texture_from_surface(&surface).unwrap();

        let texture_data =
            TextureData {
    //            surface: surface,
                texture: texture
            };

        let index = self.textures.len();
        self.textures.insert(index, texture_data);

        let out_texture =
            Texture {
                index: index,
                width: png_img.width as i32,
                height: png_img.height as i32
            };

        Ok(out_texture)
    }

    pub fn get_internal_texture(&self, texture: &Texture) -> &sdl::Texture {
        &self.textures.get(&texture.index).unwrap().texture
    }
}
