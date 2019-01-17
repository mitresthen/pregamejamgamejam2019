extern crate sdl2;
extern crate stb_image;

use stb_image::image::LoadResult;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;
use sdl2::pixels;

use stb_image::image;

mod audio_engine;
use audio_engine::AudioEngine;

fn hsl2rgb_f64(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0. { (l, l, l) }
    else {
        fn pqt2v(p: f64, q: f64, mut t: f64) -> f64 {
            if t < 0. { t += 1.; }
            if t > 1. { t -= 1.; }
            if t < (1./6.) { return p + (q - p) * 6. * t; }
            if t < (1./2.) { return q; }
            if t < (2./3.) { return p + (q - p) * (2./3. - t) * 6.; }
            return p;
        }
        let mut q =  l * (1. - s);
        if l < 0.5 { q = l * (1. + s); }
        let p = 2. * l;
        let r = pqt2v(q, p, h + 1./3.);
        let g = pqt2v(q, p, h);
        let b = pqt2v(q, p, h - 1./3.);
        (r, g, b)
    }
}

fn hsl2rgb_u8(h: u8, s: u8, l: u8) -> (u8, u8, u8) {
    let (r, g, b) = hsl2rgb_f64((h as f64) / 255., (s as f64) / 255., (l as f64) / 255.);
    ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem.window("PiplonBuzz", 1320, 768)
        .position_centered().opengl().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    let mut i = 0f64;

    let mut audio_engine = AudioEngine::new(sdl_context.audio().unwrap());

    audio_engine.play_sound_from_file("src/resources/music/personal_space.wav");

    let mut timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;


    let mut png_img = match image::load(Path::new("src/resources/image/characters.png")) {
        LoadResult::ImageU8(bytes) => { bytes },
        LoadResult::ImageF32(_) => panic!("Is float"),
        _ => panic!("Failed to load image")
    };
    
    
    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    //let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
    let temp_surface = sdl2::surface::Surface::from_data(
        &mut png_img.data,
        png_img.width as u32,
        png_img.height as u32,
        png_img.width as u32 * 4,
        sdl2::pixels::PixelFormatEnum::ABGR8888
    ).unwrap();

    let texture = texture_creator.create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let frames_per_anim = 23;
    let sprite_tile_size = (32,32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_0.center_on(Point::new(-64,120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_1.center_on(Point::new(0,240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_2.center_on(Point::new(440,360));

    // Snake - slither animation
    let mut source_rect_3 = Rect::new(0, 96, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_3 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_3.center_on(Point::new(200, 480));

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;

        // set the current frame for time
        source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

        source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

        source_rect_3.set_x(32 * ((ticks / 100 % 4)));
        dest_rect_3.set_x((1 * ((ticks / 8 ) % 768) - 672) * -1);

        canvas.clear();

        // copy the frame to the canvas
        canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false)?;
        canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false)?;
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false)?;
        canvas.copy_ex(&texture, Some(source_rect_3), Some(dest_rect_3), 0.0, None, true, false)?;

        let (r, g, b) = hsl2rgb_f64(i % 1., 1., 0.5);
        i += 0.01;
        canvas.set_draw_color(pixels::Color::RGB((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8));

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}