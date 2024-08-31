extern crate sdl2;
pub mod view;
use view::map::generate_noisemap;
use view::events;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_width = 800;
    let window_height = 600;
    let mut tile_scale = 10;
    let map_size = 300;

    let window = video_subsystem.window(
        "rust-sdl2 demo", window_width, window_height
    )
        .position_centered()
        .build()
        .expect("issue with building window");

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut map = generate_noisemap(map_size, 3, 0.1, 0.3);

    let texture_loader = canvas.texture_creator();
    let sprite_sheet = texture_loader.load_texture("assets/tileset.png").unwrap();
    for tile in map.iter() {
        tile.render(&mut canvas, &sprite_sheet, tile_scale);
    }
    canvas.present();

    let mut prev_iso_x = -1;
    let mut prev_iso_y = -1;

    // values used to offset the bound checking for mouse event
    // used since panning feature directly alters the co-ords
    let mut y_offset = 0;
    let mut x_offset = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let event_loop = events::handle_controls(
            &mut event_pump,
            &mut map,
            &mut tile_scale,
            &mut canvas,
            &sprite_sheet,
            map_size,
            window_width,
            &mut prev_iso_x,
            &mut prev_iso_y,
            &mut x_offset,
            &mut y_offset);
        if !event_loop {break 'running}
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}