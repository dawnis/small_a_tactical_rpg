use hexgametile::hexagon::HexagonalTile;
use hexgametile::terrain::Terrain;
use hexboard::TileFactory;
use std::rc::Rc;
use nannou::prelude::*;

#[derive(Clone)]
pub struct HextileFactory {
    api: &'static Draw,
}

impl HextileFactory {
    pub fn new(draw_api: &'static Draw) -> Self {
        HextileFactory {api: draw_api}
    }
}

impl TileFactory for HextileFactory {
    type Output =  HexagonalTile;

    fn api(&self) -> &Draw {
        &*self.api
    }

    //fn from_pixel(&self, scale: f32, pixel: Rgba<u8>) -> HexagonalTile {
    //   HexagonalTile::from_pixel(self.api, edge, pixel)
    //}

    fn build(&self) -> HexagonalTile {
        HexagonalTile::new(&self.api, 25., Terrain::Air)
    }

}
