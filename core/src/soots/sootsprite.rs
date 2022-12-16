use nannou::prelude::*;
use hex2d::{Coordinate, Position, Direction, Spacing};
use hexboard::GamePiece;
use crate::soots::arthropods::Arthropod;

pub struct SootSprite {
    stype: Arthropod,
    position: Position,
    texture: wgpu::Texture,
    
}

impl SootSprite {
    pub fn new(app: &App, loc: (i32, i32), stype: Arthropod) -> Self {
        SootSprite { 
            stype,
            position: Position::new(Coordinate::new(loc.0, loc.1), Direction::YZ),
            texture: stype.to_texture(app),
        }
    }

    pub fn draw(&self, draw: &Draw, scale: f32, off: Coordinate) {
        let xy_c = self.position.coord + off;
        let xy = xy_c.to_pixel(Spacing::FlatTop(scale));
        let r_sze = (3.).sqrt() * scale;
        let bb = Rect::from_w_h(r_sze, r_sze);
        draw.texture(&self.texture)
            .wh(bb.wh())
            .xy(Vec2::new(xy.0, xy.1));
    }
}

impl GamePiece for SootSprite {

    fn position(&self) -> Position {
        self.position
    }

    fn walk(&mut self) {
        let new_position = self.position + Coordinate::new(0, 1);

    }
}
