use crate::hexagonaltile::tile::HexagonalTile;
use crate::soots::sootsprite::SootSprite;
use std::cell::RefCell;
use hexboard::{Board, TileFactory};
use hex2d::Coordinate;
use nannou::prelude::*;

#[derive(Clone)]
pub struct HextileFactory<'a> {
    api: Option<&'a Draw>,
}

impl<'a> HextileFactory<'a> {
    pub fn new(api: Option<&'a Draw>) -> Self {
        HextileFactory {api}
    }
}

impl<'a> TileFactory for HextileFactory<'a> {
    type Tile =  HexagonalTile;
    type Sprite = SootSprite;

    fn draw_tile(&self, c: Coordinate, scale: f32, t: &HexagonalTile) {
        t.draw(self.api.unwrap(), c, scale)
    }

    fn draw_sprite(&self, off: Coordinate, scale: f32, s: &SootSprite) {
        s.draw(self.api.unwrap(),scale, off);
    }

    /// Draws the board using nannou.
    fn display_board(&self, board: &Board<HexagonalTile>, sprites: &[RefCell<SootSprite>], offset: (i32, i32)) {
        let offset_as_coordinate = Coordinate::new(offset.0, offset.1);

        for (loc, tile) in board.tiles.iter() {
            let oc = *loc + offset_as_coordinate;
            if board.is_viewable(oc) {
                    self.draw_tile(oc, board.scale(), tile);
               }
        }

        for sprite in sprites.iter() {
            self.draw_sprite(offset_as_coordinate, board.scale(), &sprite.borrow());
        }
    }

}
