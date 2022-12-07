use crate::hexagonaltile::tile::HexagonalTile;
use crate::terrain::Terrain;
use hexboard::{Board, TileFactory, Hextile};
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
    type Output =  HexagonalTile;

    fn draw_tile(&self, c: Coordinate, scale: f32, t: &HexagonalTile) {
        t.draw(self.api.unwrap(), c, scale)
    }

    //fn from_pixel(&self, scale: f32, pixel: Rgba<u8>) -> HexagonalTile {
    //   HexagonalTile::from_pixel(self.api, edge, pixel)
    //}

    /// Draws the board using nannou.
    fn display_board(&self, board: &Board<HexagonalTile>, offset: (i32, i32)) {
        for (loc, tile) in board.tiles.iter() {
            let oc = *loc + Coordinate::new(offset.0, offset.1);
            if board.is_viewable(oc) {
                    self.draw_tile(oc, board.scale(), tile);
               }
        }
    }

}
