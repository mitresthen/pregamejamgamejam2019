extern crate sdl2;

use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("GameJam 2019", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

    let mut events = sdl_context.event_pump().unwrap();
    let mut i = 0f64;

    'main: loop {
        let (r, g, b) = hsl2rgb_f64(i % 1., 1., 0.5);
        i += 0.0001;
        canvas.set_draw_color(pixels::Color::RGB((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8));
        canvas.clear();
        canvas.present();
        for event in events.poll_iter() {

            match event {

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    } else if keycode == Keycode::Space {
                        println!("space down");
                        canvas.present();
                    }
                }

                Event::MouseButtonDown {x, y, ..} => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

}
