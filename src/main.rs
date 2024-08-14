extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
//use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        //i = (i + 1) % 255;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        let texture_loader = canvas.texture_creator();
        let tile = texture_loader.load_texture("assets/sprite_tile.png").unwrap();
        //tile is 16 x 8 pixels
        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(Rect::new(20, 20, 10, 10)).unwrap();
        canvas.copy(&tile, None, Rect::new(100, 100, 200, 100)).unwrap();
        canvas.copy(&tile, None, Rect::new(200, 150, 200, 100)).unwrap();
        canvas.copy(&tile, None, Rect::new(300, 200, 200, 100)).unwrap();

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}