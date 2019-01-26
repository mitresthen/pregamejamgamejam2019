use std::path::Path;
use stb_image::image::LoadResult;
use stb_image::image;

use std::collections::BTreeMap;

use super::Error;

use extent::Extent;
use offset::Offset;

mod sdl {
pub use sdl2::render::TextureCreator;
pub use sdl2::render::Texture;
pub use sdl2::video::WindowContext;
pub use sdl2::surface::Surface;
pub use sdl2::pixels::PixelFormatEnum;
}

pub struct Texture {
    index: usize,
    offset: Offset,
    extent: Extent,
}

impl Texture {
    pub fn extent(&self) -> Extent { self.extent }
    pub fn offset(&self) -> Offset { self.offset }
    pub fn sub_texture(&self, offset: Offset, extent: Extent)
        -> Result<Texture, Error>
    {
        if offset.x < 0 || offset.y < 0 {
            return Err(Error::FatalError("Negative offset for sub texture".to_string()));
        }

        let max_extent = offset + extent;

        if max_extent.x > self.extent.width || max_extent.y > self.extent.height {
            return Err(Error::FatalError("Subtexture extent exceeds that of the parent texture".to_string()));
        }

        let texture =
            Texture {
                index: self.index,
                offset: self.offset + offset,
                extent: extent
            };

        Ok(texture)
    }
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
                extent: Extent::new(png_img.width as i32, png_img.height as i32),
                offset: Offset::new(),
            };

        Ok(out_texture)
    }

    pub fn get_internal_texture(&self, texture: &Texture) -> &sdl::Texture {
        &self.textures.get(&texture.index).unwrap().texture
    }
}
