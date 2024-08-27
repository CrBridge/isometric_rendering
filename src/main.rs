extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use noise::{NoiseFn, Perlin};
use rand::Rng;

#[derive(PartialEq, Debug)]
enum Terrain {
    Coal,
    Grass
}

#[derive(Debug)]
struct Tile {
    x: i32,
    y: i32,
    terrain: Terrain
}

impl Tile {
    fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<Window>,
        texture: &sdl2::render::Texture,
        scale: i32
    ) {
        let tile_x;
        let dst;
        let src: Rect = match self.terrain {
            Terrain::Grass => {
                if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {tile_x = 16;} else {tile_x = 32;};
                dst = Rect::new(
                (self.x as i32 * scale) - (self.y as i32 * scale) + (canvas.viewport().width() as i32 / 2) - (scale), 
                (self.x as i32 * scale / 2) + (self.y as i32 * scale / 2), 
                scale as u32 * 2, scale as u32 * 2);
                Rect::new(tile_x, 16, 16, 16)
            }
            Terrain::Coal => {
                dst = Rect::new(
                    (self.x as i32 * scale) - (self.y as i32 * scale) + (canvas.viewport().width() as i32 / 2) - (scale), 
                    (self.x as i32 * scale / 2) + (self.y as i32 * scale / 2) - (2 * scale), 
                    scale as u32 * 2, scale as u32 * 4);
                    Rect::new(48, 0, 16, 32)
            }
        };
        canvas.copy(&texture, src, dst).expect("Error occurred rendering tile");
    }
    fn render_outline(
        &self,
        canvas: &mut sdl2::render::Canvas<Window>,
        texture: &sdl2::render::Texture,
        scale: i32,
        outline_sprite: &mut Rect,
        map: &Vec<Tile>
    ) {
        // function renders a new outline sprite over the tile
        // also checks surrounding tiles, and edits the sprite shape if needed
        // if tile isn't visible due to surrounding tiles, does nothing
        let tile_right = map.iter().any(|f| f.x == self.x + 1 && f.y == self.y && f.terrain == Terrain::Coal);
        let tile_below = map.iter().any(|f| f.x == self.x && f.y == self.y + 1 && f.terrain == Terrain::Coal);
        let tile_right_below = map.iter().any(|f| f.x == self.x + 1 && f.y == self.y + 1 && f.terrain == Terrain::Coal);

        let mut dst = Rect::new(
            (self.x as i32 * scale) - (self.y as i32 * scale) + (canvas.viewport().width() as i32 / 2) - (scale),
            (self.x as i32 * scale / 2) + (self.y as i32 * scale / 2),
            scale as u32 * 2, scale as u32
        );

        if !tile_right && !tile_below && !tile_right_below {}
        else if tile_right && !tile_below && !tile_right_below {
            outline_sprite.set_width(outline_sprite.width() / 2);
            dst.set_width(dst.width() / 2);
        }
        else if !tile_right && tile_below && !tile_right_below {
            outline_sprite.set_width(outline_sprite.width() / 2);
            outline_sprite.set_x(outline_sprite.x() + 8);
            dst.set_width(dst.width() / 2);
            dst.set_x(dst.x() + scale);
        }
        else {return;} 

        canvas.copy(texture, *outline_sprite, dst).expect("Error occurred rendering outlines");
    }
    // altering tile co-ordinates for map movement
    fn increment_x (&mut self) {self.x += 1}
    fn decrement_x (&mut self) {self.x -= 1}
    fn increment_y (&mut self) {self.y += 1}
    fn decrement_y (&mut self) {self.y -= 1}
}

fn pixel_to_iso(x: i32, y: i32, window_width: i32, tile_scale: i32) -> (i32, i32) {
    // converts the pixel co-ordinates to its equiv value for the iso grid
    let iso_x = (((x as f32 - window_width as f32 / 2.0) / tile_scale as f32 + y as f32 * 2.0 / tile_scale as f32) / 2.0).round() as i32;
    let iso_y = ((y as f32 * 2.0 / tile_scale as f32 - (x as f32 - window_width as f32 / 2.0) / tile_scale as f32) / 2.0).round() as i32;
    (iso_x, iso_y)
}

fn generate_noisemap(map_size: i32, seed: u32, scale: f64, threshold: f64) -> Vec<Tile> {
    let perlin = Perlin::new(seed);
    let mut map: Vec<Tile> = Vec::new();
    for y in 0..map_size {
        for x in 0..map_size {
            // Sample the noise function at different points with increased scale
            let noise_value = perlin.get([x as f64 * scale, y as f64 * scale]);
            // Normalize the noise value to be between 0.0 and 1.0
            let normalized_value = (noise_value + 1.0) / 2.0;
            // Threshold the normalized noise value to get binary output
            let grid_value = if normalized_value > threshold { 1 } else { 0 };
            let terrain = match grid_value {
                0 => {Terrain::Coal}
                1 => {Terrain::Grass}
                _ => {Terrain::Grass}
            };
            map.push(Tile {
                x, y, terrain
            });
        }
    }
    return map;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_width = 800;
    let window_height = 600;
    let mut tile_scale = 20;
    let map_size = 30;

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
        canvas.present();
    }

    let mut prev_iso_x = -1;
    let mut prev_iso_y = -1;

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
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    canvas.clear();
                    tile_scale += 1;
                    for tile in map.iter() {
                        tile.render(&mut canvas, &sprite_sheet, tile_scale);
                    }
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.decrement_y();
                        tile.decrement_x();
                        tile.render(&mut canvas, &sprite_sheet, tile_scale);
                    }
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.increment_y();
                        tile.decrement_x();
                        tile.render(&mut canvas, &sprite_sheet, tile_scale);
                    }
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.increment_y();
                        tile.increment_x();
                        tile.render(&mut canvas, &sprite_sheet, tile_scale);
                    }
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.decrement_y();
                        tile.increment_x();
                        tile.render(&mut canvas, &sprite_sheet, tile_scale);
                    }
                    canvas.present();
                },
                Event::MouseMotion {x, y, ..} => {
                    // checks if the mouse has moved to a different tile, and if it has
                    // render new outlines over the current and previous tile
                    let (iso_x, iso_y) = pixel_to_iso(x, y, window_width as i32, tile_scale);
                    
                    if iso_x != prev_iso_x || iso_y != prev_iso_y {
                        if prev_iso_x >= 0 && prev_iso_y >= 0 {
                            if let Some(tile) = map.iter().find(
                                |f| f.x == prev_iso_x && f.y == prev_iso_y && f.terrain == Terrain::Grass
                            ) {
                                let mut outline_old = Rect::new(0, 24, 16, 8);
                                tile.render_outline(&mut canvas, &sprite_sheet, tile_scale, &mut outline_old, &map);
                            }
                        }
                        if iso_x >= 0 && iso_x < map_size && iso_y >= 0 && iso_y < map_size {
                            if let Some(tile) = map.iter().find(
                                |f| f.x == iso_x && f.y == iso_y && f.terrain == Terrain::Grass
                            ) {
                                let mut outline_new = Rect::new(0, 16, 16, 8);
                                tile.render_outline(&mut canvas, &sprite_sheet, tile_scale, &mut outline_new, &map);
                            }
                        }
                        prev_iso_x = iso_x;
                        prev_iso_y = iso_y;
                        canvas.present();
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}