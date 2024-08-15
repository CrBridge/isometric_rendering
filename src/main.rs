extern crate sdl2;

use rand::Rng;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::{os::windows::thread, time::Duration};
use noise::{NoiseFn, Perlin};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_width = 800;
    let window_height = 600;

    let window = video_subsystem.window(
        "rust-sdl2 demo", window_width, window_height
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    //trying random gen
    let perlin = Perlin::new(3);
    let threshold = 0.3;
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

    //loading tilemap textures
    let texture_loader = canvas.texture_creator();
    let tile = texture_loader.load_texture("assets/tileset.png").unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let tile_scale = 15;

    // Tiles are centred by adding half the screen width minus tile width
    let mut tile_x;
    for x in 0..map_size {
        for y in 0..map_size{
            if map[x][y] == 1 {
                // 50-50 chance to use either texture
                if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {tile_x = 16;} else {tile_x = 32;};
                canvas.copy(&tile, Rect::new(
                    tile_x, 16, 16, 16), 
                Rect::new(
                    (x as i32 * tile_scale) - (y as i32 * tile_scale) + (window_width as i32 / 2) - (tile_scale), 
                    (x as i32 * tile_scale / 2) + (y as i32 * tile_scale / 2), 
                    tile_scale as u32 * 2, tile_scale as u32 * 2)).unwrap();
            }
            else if map[x][y] == 0 {
                // to deal with extra height, we offset the dst rect y value and multiply the height
                canvas.copy(&tile, Rect::new(
                    48, 0, 16, 32), 
                Rect::new(
                    (x as i32 * tile_scale) - (y as i32 * tile_scale) + (window_width as i32 / 2) - (tile_scale), 
                    (x as i32 * tile_scale / 2) + (y as i32 * tile_scale / 2) - (2 * tile_scale), 
                    tile_scale as u32 * 2, tile_scale as u32 * 4)).unwrap();
            }
            canvas.present();
        }
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // if quit or keydown events are true (e.g., window is closed or escape key pressed)
                // loop is broken which lets the code finish
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Mouse movement event
                Event::MouseMotion { 
                    timestamp, 
                    window_id, 
                    which, 
                    mousestate, 
                    x, 
                    y, 
                    xrel, 
                    yrel 
                } => {
                    // math here is supposed to do the opposite of above
                    // instead of calculating screen co-ords from a grid, we need to reverse
                    // the mouses screen co-ords to its hypothetical grid co-ords to find out
                    // which tile it is hovering over
                    let mouse_x = ((x - window_width as i32 / 2) / tile_scale + y * 2 / tile_scale) / 2;
                    let mouse_y = (y * 2 / tile_scale - (x - window_width as i32 / 2) / tile_scale) / 2;
                
                    // if the calculated hypothetical grid co-ords co-incide with an existing surface-level tile,
                    // we render a red outline for feedback
                    // (currently, we render the outline and its rendered forever, but thats just for now)
                    if mouse_x >= 0 && mouse_x <= map_size as i32 - 1 && 
                    mouse_y >= 0 && mouse_y <= map_size as i32 - 1 &&
                    map[mouse_x as usize][mouse_y as usize] == 1{
                        canvas.copy(&tile, Rect::new(
                            0, 16, 16, 8), 
                        Rect::new(
                            (mouse_x as i32 * tile_scale) - (mouse_y as i32 * tile_scale) + (window_width as i32 / 2) - (tile_scale), 
                            (mouse_x as i32 * tile_scale / 2) + (mouse_y as i32 * tile_scale / 2), 
                            tile_scale as u32 * 2, tile_scale as u32)).unwrap();
                    }
                },
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}