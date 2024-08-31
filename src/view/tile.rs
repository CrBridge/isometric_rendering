use sdl2::video::Window;
use sdl2::rect::Rect;

#[derive(PartialEq, Debug)]
pub enum Terrain {
    Coal,
    Grass,
    Flowers
}

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub terrain: Terrain
}

impl Tile {
    pub fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<Window>,
        texture: &sdl2::render::Texture,
        scale: i32
    ) {
        let dst;
        let src: Rect = match self.terrain {
            Terrain::Grass => {
                dst = Rect::new(
                (self.x as i32 * scale) - (self.y as i32 * scale) + (canvas.viewport().width() as i32 / 2) - (scale), 
                (self.x as i32 * scale / 2) + (self.y as i32 * scale / 2), 
                scale as u32 * 2, scale as u32 * 2);
                Rect::new(32, 16, 16, 16)
            }
            Terrain::Flowers => {
                dst = Rect::new(
                (self.x as i32 * scale) - (self.y as i32 * scale) + (canvas.viewport().width() as i32 / 2) - (scale), 
                (self.x as i32 * scale / 2) + (self.y as i32 * scale / 2), 
                scale as u32 * 2, scale as u32 * 2);
                Rect::new(16, 16, 16, 16)
            }
            Terrain::Coal => {
                dst = Rect::new(
                (self.x as i32 * scale) - (self.y as i32 * scale) + (canvas.viewport().width() as i32 / 2) - (scale), 
                (self.x as i32 * scale / 2) + (self.y as i32 * scale / 2) - (scale), 
                scale as u32 * 2, scale as u32 * 3);
                Rect::new(48, 8, 16, 24)
            }
        };
        canvas.copy(&texture, src, dst).expect("Error occurred rendering tile");
    }
    pub fn render_outline(
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
    //pub fn increment_x (&mut self) {self.x += 1}
    //pub fn decrement_x (&mut self) {self.x -= 1}
    //pub fn increment_y (&mut self) {self.y += 1}
    //pub fn decrement_y (&mut self) {self.y -= 1}
    pub fn move_tiles (&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}