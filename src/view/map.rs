use super::tile::{Tile, Terrain};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn generate_noisemap(map_size: i32, seed: u32, scale: f64, threshold: f64) -> Vec<Tile> {
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
                1 => {
                    if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {Terrain::Grass} else {Terrain::Flowers}
                }
                _ => {Terrain::Grass}
            };
            map.push(Tile {
                x, y, terrain
            });
        }
    }
    return map;
}