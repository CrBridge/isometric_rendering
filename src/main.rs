extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use noise::{NoiseFn, Perlin};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        //.resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    //trying random gen
    let perlin = Perlin::new(1);
    let threshold = 0.5;
    let scale = 0.2;
    let map_size = 50;
    let mut map: Vec<Vec<i32>> = Vec::new();
    for y in 0..map_size {
        let mut map_row: Vec<i32> = Vec::new();
        for x in 0..map_size {
            // Sample the noise function at different points with increased scale
            let noise_value = perlin.get([x as f64 * scale, y as f64 * scale]);
            // Normalize the noise value to be between 0.0 and 1.0
            let normalized_value = (noise_value + 1.0) / 2.0;
            // Threshold the normalized noise value to get binary output
            let grid_value = if normalized_value > threshold { 1 } else { 0 };
            map_row.push(grid_value);
        }
        map.push(map_row);
    }
    //end of rand gen

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
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
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let tile_scale = 50;

        // When I want more tiles, I should have all of them in one tilemap png,
        // for individual tiles I can just pass the co-ords to src
        for x in 0..map_size {
            for y in 0..map_size{
                if map[x][y] == 1 {
                    canvas.copy(&tile, None, Rect::new(
                        (x as i32 * tile_scale) - (y as i32 * tile_scale), 
                        (x as i32 * tile_scale / 2) + (y as i32 * tile_scale / 2), 
                        tile_scale as u32 * 2, tile_scale as u32)).unwrap();
                    //canvas.draw_line((0, 0), (x * 100, y * 100)).unwrap();
                }
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}