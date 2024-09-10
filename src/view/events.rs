use super::tile::{Tile, Terrain};
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::rect::Rect;

fn pixel_to_iso(x: i32, y: i32, window_width: i32, tile_scale: i32) -> (i32, i32) {
    // converts the pixel co-ordinates to its equiv value for the iso grid
    let iso_x = (((x as f32 - window_width as f32 / 2.0) / tile_scale as f32 + y as f32 * 2.0 / tile_scale as f32) / 2.0).round() as i32;
    let iso_y = ((y as f32 * 2.0 / tile_scale as f32 - (x as f32 - window_width as f32 / 2.0) / tile_scale as f32) / 2.0).round() as i32;
    (iso_x, iso_y)
}

pub fn handle_controls(
    event_pump: &mut EventPump,
    map: &mut Vec<Tile>,
    tile_scale: &mut i32,
    canvas: &mut sdl2::render::Canvas<Window>,
    texture: &sdl2::render::Texture,
    map_size: i32,
    window_width: u32,
    prev_iso_x: &mut i32,
    prev_iso_y: &mut i32,
    x_offset: &mut i32,
    y_offset: &mut i32
    ) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                // if quit or keydown events are true (e.g., window is closed or escape key pressed)
                // loop is broken which lets the code finish
                Event::Quit {..} |
                Event::KeyDown {  keycode: Some(Keycode::Escape), .. } => {
                    return false;
                },
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    if *tile_scale < 60 {
                        canvas.clear();
                        *tile_scale += 1;
                        for tile in map.iter() {
                            tile.render(canvas, texture, *tile_scale);
                        }
                        canvas.present();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    if *tile_scale > 3 {
                        canvas.clear();
                        *tile_scale -= 1;
                        for tile in map.iter() {
                            tile.render(canvas, texture, *tile_scale);
                        }
                        canvas.present();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.move_tiles(1, 1);
                        tile.render(canvas, texture, *tile_scale);
                    }
                    *y_offset += 1;
                    *x_offset += 1;
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.move_tiles(1, -1);
                        tile.render(canvas, texture, *tile_scale);
                    }
                    *y_offset -= 1;
                    *x_offset += 1;
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.move_tiles(-1, -1);
                        tile.render(canvas, texture, *tile_scale);
                    }
                    *y_offset -= 1;
                    *x_offset -= 1;
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    canvas.clear();
                    for tile in map.iter_mut() {
                        tile.move_tiles(-1, 1);
                        tile.render(canvas, texture, *tile_scale);
                    }
                    *y_offset += 1;
                    *x_offset -= 1;
                    canvas.present();
                },
                Event::MouseMotion {x, y, ..} => {
                    // checks if the mouse has moved to a different tile, and if it has
                    // render new outlines over the current and previous tile
                    let (iso_x, iso_y) = pixel_to_iso(x, y, window_width as i32, *tile_scale);

                    if (iso_x != *prev_iso_x || iso_y != *prev_iso_y) && *tile_scale >= 8 {
                        if *prev_iso_x >= (0 + *x_offset) && *prev_iso_x < (map_size + *x_offset) && *prev_iso_y >= (0 + *y_offset) && *prev_iso_y < (map_size + *y_offset) {
                            if let Some(tile) = map.iter().find(
                                |f| f.x == *prev_iso_x && f.y == *prev_iso_y && f.terrain != Terrain::Coal
                            ) {
                                let mut outline_old = Rect::new(0, 24, 16, 8);
                                tile.render_outline(canvas, texture, *tile_scale, &mut outline_old, &map);
                            }
                        }
                        if iso_x >= (0 + *x_offset) && iso_x < (map_size + *x_offset) && iso_y >= (0 + *y_offset) && iso_y < (map_size + *y_offset) {
                            if let Some(tile) = map.iter().find(
                                |f| f.x == iso_x && f.y == iso_y && f.terrain != Terrain::Coal
                            ) {
                                let mut outline_new = Rect::new(0, 16, 16, 8);
                                tile.render_outline(canvas, texture, *tile_scale, &mut outline_new, &map);
                            }
                        }
                        *prev_iso_x = iso_x;
                        *prev_iso_y = iso_y;
                        canvas.present();
                    }
                },
                _ => {}
            }
        }
        true
}